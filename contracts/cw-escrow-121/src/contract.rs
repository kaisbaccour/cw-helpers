use cosmwasm_std::{
    ensure_eq, entry_point, to_binary, BankMsg, Deps, DepsMut, Env, MessageInfo, QueryResponse,
    Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Party, CONFIG};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let party_a = deps
        .api
        .addr_validate(&msg.party_a)
        .map_err(|_| ContractError::InvalidAddress)?;
    let party_b = deps
        .api
        .addr_validate(&msg.party_b)
        .map_err(|_| ContractError::InvalidAddress)?;
    CONFIG.save(
        deps.storage,
        &Config {
            party_a: Party {
                address: party_a,
                funds: msg.party_a_funds,
                deposited: false,
            },
            party_b: Party {
                address: party_b,
                funds: msg.party_b_funds,
                deposited: false,
            },
        },
    )?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("party_a", msg.party_a)
        .add_attribute("party_b", msg.party_b))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    only_authorized_party(deps.as_ref(), info.clone())?;
    match msg {
        ExecuteMsg::Deposit {} => execute_deposit(deps, info),
        ExecuteMsg::Exchange {} => execute_exchange(deps),
        ExecuteMsg::Withdraw {} => execute_withdraw(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    let response = match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?)?,
    };
    Ok(response)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

fn execute_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let party_address = info.sender;
    let mut new_config = config.clone();

    match party_address.clone() {
        a if a == config.party_a.address => {
            ensure_eq!(
                info.funds,
                config.party_a.funds,
                ContractError::FundsNotEqualToConfig
            );
            ensure_eq!(
                false,
                config.party_a.deposited,
                ContractError::FundsAlreadyDeposited
            );
            new_config.party_a.deposited = true;
        }
        b if b == config.party_b.address => {
            ensure_eq!(
                info.funds,
                config.party_b.funds,
                ContractError::FundsNotEqualToConfig
            );
            ensure_eq!(
                false,
                config.party_b.deposited,
                ContractError::FundsAlreadyDeposited
            );
            new_config.party_b.deposited = true;
        }
        _ => return Err(ContractError::InvalidAddress),
    };

    CONFIG.save(deps.storage, &new_config)?;

    let response = Response::new()
        .add_attribute("action", "deposit")
        .add_attribute("deposited_by", party_address.to_string());

    Ok(response)
}

fn execute_withdraw(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let party_address = info.sender;
    let mut new_config = config.clone();

    match party_address.clone() {
        a if a == config.party_a.address => {
            ensure_eq!(
                true,
                config.party_a.deposited,
                ContractError::FundsHaventBeenDeposited
            );
            new_config.party_a.deposited = false;
        }
        b if b == config.party_b.address => {
            ensure_eq!(
                true,
                config.party_b.deposited,
                ContractError::FundsHaventBeenDeposited
            );
            new_config.party_b.deposited = false;
        }
        _ => return Err(ContractError::InvalidAddress),
    };

    CONFIG.save(deps.storage, &new_config)?;

    let response = Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("withdrawn by", party_address.to_string());

    Ok(response)
}

fn execute_exchange(deps: DepsMut) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut new_config = config.clone();

    // Check that party A deposited the funds
    ensure_eq!(
        true,
        config.party_a.deposited,
        ContractError::FundsHaventBeenDeposited
    );
    // Check that party B deposited the funds
    ensure_eq!(
        true,
        config.party_b.deposited,
        ContractError::FundsHaventBeenDeposited
    );
    new_config.party_a.deposited = false;
    new_config.party_b.deposited = false;

    CONFIG.save(deps.storage, &new_config)?;

    let response = Response::new()
        .add_attribute("action", "exchange")
        .add_message(BankMsg::Send {
            to_address: config.party_a.address.to_string(),
            amount: config.party_b.funds,
        })
        .add_message(BankMsg::Send {
            to_address: config.party_b.address.to_string(),
            amount: config.party_a.funds,
        });

    Ok(response)
}

fn only_authorized_party(deps: Deps, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // check the calling address is either party_a or party_b
    if info.sender != config.party_a.address && info.sender != config.party_b.address {
        return Err(ContractError::Unauthorized);
    }

    Ok(Response::default())
}

#[cfg(test)]
mod tests {

    use crate::msg::{ConfigResponse, QueryMsg};

    use super::*;
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, Coin, OwnedDeps,
    };

    const PARTY_A: &str = "alice";
    const PARTY_B: &str = "bob";

    fn setup() -> (OwnedDeps<MockStorage, MockApi, MockQuerier>, InstantiateMsg) {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            party_a: PARTY_A.to_string(),
            party_b: PARTY_B.to_string(),
            party_a_funds: vec![Coin::new(1_000000, "unois"), Coin::new(2, "btc")],
            party_b_funds: vec![Coin::new(5_000000, "ujuno")],
        };
        let res = instantiate(deps.as_mut(), mock_env(), msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());
        (deps, msg)
    }

    #[test]
    fn instantiate_works() {
        let (mut deps, msg) = setup();
        let res = instantiate(deps.as_mut(), mock_env(), msg).unwrap();
        assert_eq!(0, res.messages.len());
        let config: ConfigResponse =
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()).unwrap();
        assert_eq!(
            config,
            ConfigResponse {
                party_a: Party {
                    address: Addr::unchecked(PARTY_A),
                    funds: vec![Coin::new(1_000000, "unois"), Coin::new(2, "btc")], // TODO permutation of coins should be accepted, order shouldn't matter
                    deposited: false
                },
                party_b: Party {
                    address: Addr::unchecked(PARTY_B),
                    funds: vec![Coin::new(5_000000, "ujuno")],
                    deposited: false
                },
            }
        );
    }
    #[test]
    fn only_parties_can_deposit_withdraw_or_exchange() {
        let (mut deps, msg) = setup();
        instantiate(deps.as_mut(), mock_env(), msg).unwrap();
        // Deposit
        let msg = ExecuteMsg::Deposit {};
        let err = execute(
            deps.as_mut(),
            mock_info(
                "some_random_person",
                &[Coin::new(1_000000, "unois"), Coin::new(3, "btc")],
            ),
            msg,
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::Unauthorized));
        // Withdraw
        let msg = ExecuteMsg::Withdraw {};
        let err = execute(deps.as_mut(), mock_info("some_random_person", &[]), msg).unwrap_err();
        assert!(matches!(err, ContractError::Unauthorized));
        // Exchange
        let msg = ExecuteMsg::Exchange {};
        let err = execute(deps.as_mut(), mock_info("some_random_person", &[]), msg).unwrap_err();
        assert!(matches!(err, ContractError::Unauthorized));
    }
    #[test]
    fn parties_need_to_deposit_exact_amount() {
        let (mut deps, msg) = setup();
        instantiate(deps.as_mut(), mock_env(), msg).unwrap();
        let msg = ExecuteMsg::Deposit {};
        // Party A
        let err = execute(
            deps.as_mut(),
            mock_info(
                PARTY_A,
                &[Coin::new(1_000000, "unois"), Coin::new(3, "btc")],
            ),
            msg.clone(),
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::FundsNotEqualToConfig));

        // Check on party B
        let err = execute(
            deps.as_mut(),
            mock_info(PARTY_B, &[Coin::new(4_000000, "ujuno")]),
            msg,
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::FundsNotEqualToConfig));
    }
    #[test]
    fn parties_cannot_deposit_twice() {
        let (mut deps, msg) = setup();
        instantiate(deps.as_mut(), mock_env(), msg).unwrap();
        let msg = ExecuteMsg::Deposit {};
        execute(
            deps.as_mut(),
            mock_info(
                PARTY_A,
                &[Coin::new(1_000000, "unois"), Coin::new(2, "btc")],
            ),
            msg.clone(),
        )
        .unwrap();
        let err = execute(
            deps.as_mut(),
            mock_info(
                PARTY_A,
                &[Coin::new(1_000000, "unois"), Coin::new(2, "btc")],
            ),
            msg.clone(),
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::FundsAlreadyDeposited));
        // Check on party B
        execute(
            deps.as_mut(),
            mock_info(PARTY_B, &[Coin::new(5_000000, "ujuno")]),
            msg.clone(),
        )
        .unwrap();
        let err = execute(
            deps.as_mut(),
            mock_info(PARTY_B, &[Coin::new(5_000000, "ujuno")]),
            msg,
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::FundsAlreadyDeposited));
    }
    #[test]
    fn parties_cannot_withdraw_if_they_did_not_deposit() {}
    #[test]
    fn exchange_fails_if_missing_one_depositor() {}
    #[test]
    fn withdraw_works_even_when_other_party_deposited() {}
    #[test]
    fn the_exchange_process_can_work_more_than_once() {}
    #[test]
    fn withdraw_works() {}
    #[test]
    fn deposit_works() {}
    #[test]
    fn exchange_works() {}
}
