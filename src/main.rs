#[derive(Copy, Clone)]
pub struct OptionFamily;

pub trait OneTypeParam<A>: Copy {
    type This;
}

trait Functor<A, B>: OneTypeParam<A> + OneTypeParam<B> {
    fn map<F>(self, this: This<Self, A>, f: F) -> This<Self, B>
    where
        F: Fn(A) -> B + Copy;
}

trait Monad<A, B>: Functor<A, B> {
    fn bind<F>(self, a: This<Self, A>, f: F) -> This<Self, B>
    where
        F: Fn(A) -> This<Self, B> + Copy;
}

impl<A> OneTypeParam<A> for OptionFamily {
    type This = Option<A>;
}

impl<A, B> Functor<A, B> for OptionFamily {
    fn map<F>(self, this: This<Self, A>, f: F) -> This<Self, B>
    where
        F: Fn(A) -> B + Copy {
        // I'm not cheating!
        this.map(f)
    }
}

impl<A, B> Monad<A, B> for OptionFamily {
    fn bind<F>(self, this: This<Self, A>, f: F) -> This<Self, B>
    where
        F: Fn(A) -> This<Self, B> + Copy {
        // It fits 😉
        this.and_then(f)
    }
}

pub type This<T, A> = <T as OneTypeParam<A>>::This;

fn compose_monad<M, F, G, A, B, C>(
    monad: M,
    f: F,
    g: G
) -> impl FnOnce(A) -> This<M, C>
where
    M: Monad<A, B> + Monad<B, C>,
    F: FnOnce(A) -> This<M, B>,
    G: FnOnce(B) -> This<M, C>,
{
    move |a| f(a).bind(monad, g)
}

fn main() {
    println!("Hello, world!");
    let plus_one_times_two = compose_monad(
        OptionFamily,
        |x: u32| x.checked_add(1),
        |x: u32| x.checked_mul(2)
    );

    assert_eq!(plus_one_times_two(2), Some(6));
}
