use std::{error::Error, fmt::Debug, time::Duration};
use passwords::PasswordGenerator;
use headless_chrome::Browser;
use names::{Generator, Name};
use mockd::datetime;
use headless_chrome::protocol::cdp::Page;


fn main() -> Result<(), Box<dyn Error>>{
	let browser = Browser::default()?;
	let username = generate_name();
	let password = generate_password();
	let date_of_birth = generate_date();
	//create_yopmail_account(&browser, &username);
	create_tiktok_account(&browser, &date_of_birth, &username, &password)
}

fn generate_date() -> Vec<String> {
	let month = datetime::month();
	let day = datetime::day();
	let year = datetime::year();
	let result = vec![day, month, year];
	println!("Date is: {0}.{1}.{2}", result[0], result[1], result[2]);
	return result
}

fn generate_password() -> String{
	let pg = PasswordGenerator {
		length: 8,
		numbers: true,
		lowercase_letters: true,
		uppercase_letters: true,
		symbols: true,
		spaces: false,
		exclude_similar_characters: true,
		strict: true,
	};
	let password = pg.generate_one().unwrap();
	println!("generated password: {password}");
	return password;
}

fn generate_name() -> String{
	let mut generator = Generator::with_naming(Name::Numbered);
	let name = generator.next().unwrap();
	println!("Created name {name}");
	return  name + "@yopmail.com";
}

fn create_yopmail_account(
	browser: &Browser,
	username: &String
) -> Result<(), Box<dyn Error>>{
	let tab = browser.new_tab()?;
	tab.navigate_to("https://www.yopmail.com/en/")?;
	tab.wait_for_element("input#login")?
				.type_into(username.as_str())?;
	tab.press_key("Enter")?;
	Ok(())
}

fn create_tiktok_account(
	browser: &Browser,
	date: &Vec<String>,
	username: &String,
	password: &String) -> Result<(), Box<dyn Error>> {
	let tab = browser.new_tab()?;
	let month_elem = "#Month-options-item-".to_owned() + date[1].as_str();
	let day_elem = "#Day-options-item-".to_owned() + date[0].as_str();
	let year_num = (2023 - date[2].parse::<i32>().unwrap()).to_string();
	let year_elem = "#Year-options-item-".to_owned() + year_num.as_str();
	//navigate to signup
	tab.navigate_to("https://www.tiktok.com/signup")?;
	//click to phone/email option
	tab.wait_for_element(".e1cgu1qo0")?.click()?;
	//click to swap for email
	tab.wait_for_element(".epl6mg0")?.click()?;

	//set month
	tab.wait_for_element(r#"[aria-label="Month. Double-tap for more options"]"#)?
		.click()?;
	tab.wait_for_element(month_elem.as_str())?.scroll_into_view()?
		.click()?;

	//set day
	tab.find_element(r#"[aria-label="Day. Double-tap for more options"]"#)?
		.scroll_into_view()?
		.click()?;
	tab.wait_for_element(day_elem.as_str())?.scroll_into_view()?.click()?;

	//set year
	tab.find_element(r#"[aria-label="Year. Double-tap for more options"]"#)?
		.scroll_into_view()?
		.click()?;
	tab.wait_for_element(year_elem.as_str())?.scroll_into_view()?.click()?;

	//type in email
	tab.find_element(r#"[name="email"]"#)?
		.scroll_into_view()?
		.type_into(username)?;
	//type in password
	tab.find_element(r#"[type="password"]"#)?
		.scroll_into_view()?
		.type_into(password)?;
	//send code to the email
	tab.wait_for_element_with_custom_timeout(r#"[data-e2e="send-code-button"]"#, Duration::from_secs(2))?
		.scroll_into_view()?
		.click()?;
  let jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        None,
        true)?;
    // Save the screenshot to disc
    std::fs::write("screenshot.jpeg", jpeg_data)?;
	Ok(())
}
