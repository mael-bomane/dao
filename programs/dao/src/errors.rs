use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("DAO Name Empty.")]
    DAONameEmpty,
    #[msg("DAO Name Too Long, 50 Characters Max.")]
    DAONameTooLong,
    #[msg("Threshold Error, 50% to 100%.")]
    ThresholdError,
    #[msg("Wrong DAO Mint.")]
    WrongDAOMint,
    #[msg("Signer Not Program Upgrade Authority.")]
    SignerNotProgramUpgradeAuthority,
    #[msg("This Protocol Not Program Upgrade Authority.")]
    ProtocolNotProgramUpgradeAuthority,
    #[msg("Program Already In Protocol List.")]
    ProgramAlreadyInProtocolList,
    #[msg("Program Not In Protocol List.")]
    ProgramNotInProtocolList,
    #[msg("Wrong Program ID.")]
    WrongProgramID,
    #[msg("Mismatch Protocol Owner And Program Owner.")]
    MismatchProtocolOwnerAndProgramOwner,
    #[msg("No Mint Provided.")]
    NoMintProvided,
}
