pub const PROXY_LIST: [&'static str; 1] = [
    "https://raw.githubusercontent.com/opengs/uashieldtargets/v2/proxy.json",
];

pub const SITES_ORIGINS: [&'static str; 2] = [
    "https://gitlab.com/jacobean_jerboa/sample/-/raw/main/sample",
    "https://raw.githubusercontent.com/opengs/uashieldtargets/v2/sites.json",
];

pub const DEFAULT_TARGETS: [&'static str; 33] = [
    "https://www.kp.ru/",
    "https://ria.ru/",
    "https://lenta.ru/",
    "https://www.mk.ru/",
    "https://rg.ru/",
    "https://www.gazeta.ru/",
    "https://aif.ru/",
    "https://tass.ru/",
    "https://sber.ru/",
    "https://3dsec.sberbank.ru/",
    "https://cc-host02.sbercontact.sberbank.ru/agentdesktop/",
    "https://developer.sberbank.ru",
    "https://idppsi.sberbank.ru",
    "https://kurs-mobile.sberbank.ru",
    "https://meetup.sberbank.ru",
    "https://office.sberbank.ru",
    "https://hr-ift.sberbank.ru/",
    "https://userid.sber.ru/",
    "https://sberchat.sberbank.ru/",
    "https://web1.online.sberbank.ru",
    "https://webquik.sberbank.ru",
    "https://sberprime.sber.ru/",
    "https://platformv.sber.ru/",
    "https://friend.sber.ru/",
    "https://beta.ai.sber.ru/",
    "http://sbermail-cloud.sber.ru/",
    "https://pking.sberbank.ru/",
    "https://sberapi.sbercontact.sberbank.ru/agentdesktop/",
    "https://sber247.sbercontact.sberbank.ru/",
    "https://sbercontact.sberbank.ru/",
    "https://pmcloud.sberbank.ru/",
    "https://online.sberbank.ru/",
    "https://apps.sberbank.ru/"
];


pub const READ_TIMEOUT_SECONDS: u64 = 10;
pub const MAX_REQUESTS_TO_SITE: u64 = 100;
pub const MAX_WORKERS: usize = 600; // MAX_JOBS*1.2
pub const MAX_JOBS: usize = 500;
