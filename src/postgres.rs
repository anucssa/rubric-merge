use crate::qpay::QPayMember;
use color_eyre::eyre::OptionExt as _;
use color_eyre::Result;
use eyre::Context as _;
use itertools::Itertools;

impl QPayMember {
    pub fn in_membership_db(&self, db: &mut postgres::Client, table: &str) -> bool {
        let result = db.query_one(
            &format!("SELECT * FROM {table} WHERE email = $1"),
            &[&self.email],
        );
        result.is_ok()
    }

    pub fn create_membership(&self, db: &mut postgres::Client, table: &str) -> Result<()> {
        let mut name_words = self.fullname.split(' ');
        let first_name = name_words.next().ok_or_eyre("Failed to find first_name")?;
        let last_name = name_words.into_iter().join(" ");

        let uid = {
            let owned_uid = self
                .responses
                .get("Student Number")
                .ok_or_eyre("No response to question")?
                .to_ascii_lowercase();

            let mut uid = owned_uid.as_str();

            if uid.starts_with("u") {
                uid = &uid[1..];
            }

            uid.parse::<i64>()
                .with_context(|| "Could not parse uid to u32")
        }
        .with_context(|| "getting uid")?;

        let source = match self.membershiptype.as_str() {
            "Free Membership" => Some("qpay free"),
            "Supporter Membership" => Some("qpay payed"),
            _ => None,
        };

        let query = format!(
            "INSERT INTO {table} (first_name, last_name, uid, email, qpay_member_id, membership_source, age_range, gender, year_of_study, student_category) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                    );

        let _result = db
            .query(
                &query,
                &[
                    &first_name,
                    &last_name,
                    &uid,
                    &self.email,
                    &self.membershipid,
                    &source,
                    &self.responses.get("Age Range"),
                    &self.responses.get("Your Gender"),
                    &self.responses.get("What year of study are you in?"),
                    &self
                        .responses
                        .get("Please select the relevant category that best applies to you"),
                ],
            )
            .with_context(|| "Inserting")?;

        Ok(())
    }
}
