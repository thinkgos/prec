use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Surplus {
    // 序号
    pub id: String,
    // 交易网会员代码
    pub tran_net_member_code: String,
    // 子账号
    pub sub_account_no: String,
    // 金额
    pub amount: String,
}

impl FromStr for Surplus {
    type Err = anyhow::Error;

    // 78&12345678&5609000000000001&488305.60&
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vs = s.split("&");

        let id = vs.next().ok_or(anyhow!("未找到序号"))?.to_owned();
        let tran_net_member_code = vs.next().ok_or(anyhow!("未找到交易网会员代码"))?.to_owned();
        let sub_account_no = vs.next().ok_or(anyhow!("未找到子账号"))?.to_owned();
        let amount = vs.next().ok_or(anyhow!("未找到金额"))?.to_owned();

        Ok(Self {
            id,
            tran_net_member_code,
            sub_account_no,
            amount,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Surplus;

    #[test]
    fn parse_string_to_surplus() -> Result<(), anyhow::Error> {
        let s = "78&12345678&5609000000000001&488305.60&";
        let v: Surplus = s.parse()?;
        assert_eq!(
            v,
            Surplus {
                id: "78".to_owned(),
                tran_net_member_code: "12345678".to_owned(),
                sub_account_no: "5609000000000001".to_owned(),
                amount: "488305.60".to_owned(),
            }
        );
        Ok(())
    }
}
