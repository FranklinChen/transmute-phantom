mod user {
    use std::marker::PhantomData;
    use std::mem;

    pub struct Unauthenticated;
    pub struct Authenticated;

    pub struct User<T> {
        id: usize,
        _phantom: PhantomData<T>
    }

    // Methods available for all users.
    impl<T> User<T> {
        pub fn print_hello(&self) {
            println!("User {} says hello!", self.id);
        }
    }

    // Methods available only for unauthenticated.
    impl User<Unauthenticated> {
        pub fn new(id: usize) -> Self {
            User {
                id: id,
                _phantom: PhantomData
            }
        }

        pub fn print_id(&self) {
            println!("Unauthenticated user {}", self.id);
        }

        /// Sneaky way of reusing the exact same object as a different type.
        pub fn authenticate(self) -> User<Authenticated> {
            // Do actual authentication.
            println!("Authenticating user {}...", self.id);

            // Then return self.
            unsafe { mem::transmute(self) }
        }
    }

    // Methods available only for authenticated.
    impl User<Authenticated> {
        pub fn print_id(&self) {
            println!("Authenticated user {}", self.id);
        }

        pub fn print_secret(&self) {
            println!("Authenticated user {} knows a secret!", self.id);
        }

        pub fn log_out(self) -> User<Unauthenticated> {
            println!("Authenticated user {} logging out...", self.id);
            unsafe { mem::transmute(self) }
        }
    }
}

use user::*;

fn main() {
    // Type error: cannot construct User<Authenticated>
    //let impossible_authenticated_user: User<Authenticated> = User::new(2);

    // Privacy error: User struct fields are private.
    //let cannot_create_user_without_new: User<Unauthenticated> =
    //    User { id: 2, _phantom : std::marker::PhantomData };

    // The only way to create a user is to create an unauthenticated one
    // using constructor.
    let unauthenticated_user: User<Unauthenticated> = User::new(5);
    unauthenticated_user.print_hello();
    unauthenticated_user.print_id();

    // Move unauthenticated_user.
    let authenticated_user: User<Authenticated> = unauthenticated_user.authenticate();
    authenticated_user.print_hello();
    authenticated_user.print_id();
    authenticated_user.print_secret();

    // Ownership type error because unauthenticated_user was moved.
    //unauthenticated_user.print_id();

    // Type error.
    //unauthenticated_user.print_secret();

    let logged_out_user: User<Unauthenticated> = authenticated_user.log_out();

    // Type error.
    //logged_out_user.print_secret();

    // Ownership type error.
    //authenticated_user.print_secret();
}
