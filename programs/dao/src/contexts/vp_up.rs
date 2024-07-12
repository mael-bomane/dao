pub fn deposit(&mut self, amount: u64) -> Result<()> {
    let accounts = Transfer {
        from: self.signer_ata.to_account_info(),
        to: self.vp_vault.to_account_info(),
        authority: self.creator.to_account_info(),
    };

    let cpi = CpiContext::new(self.token_program.to_account_info(), accounts);

    transfer(cpi, amount)
}
