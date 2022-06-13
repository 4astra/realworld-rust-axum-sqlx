use conduit_core::profiles::repository::MockProfilesRepository;
use conduit_core::users::repository::MockUsersRepository;
use conduit_core::utils::security_service::MockSecurityService;
use conduit_core::utils::token_service::MockTokenService;

#[derive(Default)]
pub struct UsersServiceTestFixture {
    pub mock_repository: MockUsersRepository,
    pub mock_token_service: MockTokenService,
    pub mock_security_service: MockSecurityService,
}

#[derive(Default)]
pub struct ProfilesServiceTestFixture {
    pub mock_profiles_repository: MockProfilesRepository,
    pub mock_users_repository: MockUsersRepository,
}

impl UsersServiceTestFixture {
    pub fn new() -> Self {
        Self {
            mock_repository: MockUsersRepository::new(),
            mock_token_service: MockTokenService::new(),
            mock_security_service: MockSecurityService::new(),
        }
    }
}

impl ProfilesServiceTestFixture {
    pub fn new() -> Self {
        Self {
            mock_profiles_repository: MockProfilesRepository::new(),
            mock_users_repository: MockUsersRepository::new(),
        }
    }
}
