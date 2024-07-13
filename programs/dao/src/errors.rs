use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("DAO Name Empty.")]
    DAONameEmpty,
    #[msg("DAO Name Too Long, 50 Characters Max.")]
    DAONameTooLong,
    #[msg("Poll Title Empty.")]
    PollTitleEmpty,
    #[msg("Poll Title Too Long, 50 Characters Max.")]
    PollTitleTooLong,
    #[msg("Poll Content Empty.")]
    PollContentEmpty,
    #[msg("Poll Content Too Long, 280 Characters Max.")]
    PollContentTooLong,
    #[msg("Threshold Error, 50% to 100%.")]
    ThresholdError,
    #[msg("Wrong DAO Mint.")]
    WrongDAOMint,
    #[msg("Not Enough Deposits To Start Poll.")]
    NotEnoughDepositsToStartPoll,
    #[msg("No Deposits For This User In This DAO.")]
    NoDepositsForThisUserInThisDAO,
    #[msg("No Voting Power For This User Found In This DAO.")]
    NoVotingPowerForThisUserFoundInThisDAO,
    #[msg("Program Not In Protocol List.")]
    ProgramNotInProtocolList,
    #[msg("Wrong Program ID.")]
    WrongProgramID,
    #[msg("Mismatch Protocol Owner And Program Owner.")]
    MismatchProtocolOwnerAndProgramOwner,
    #[msg("No Mint Provided.")]
    NoMintProvided,
}
