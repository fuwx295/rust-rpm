pub struct Package {
    pub name: String,
    pub epoch: Option<i32>,
    pub version: String,
    pub release: String,
    pub arch: Option<String>,
    pub license: String,
    pub summary: String,
    pub description: String,
    pub buildtimee: i32,
}
