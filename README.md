This repo contains 3 implementations of RustMart examples, based on the original implementation: [rustmart-yew-example].

Originally, I only intended to port the RustMart example from Yew, just as a little experiment, to Spair. I found that the size of `.wasm` file of the Spair app is more than 50% smaller than the Yew app, which is not in my expectation:

| Implemenation          | Size of `.wasm` |
| ---------------------- | --------------- |
| [rustmart-yew-example] | 477.7kb         |
| rustmart-spair         | 173.6kb         |

I wonder why? I guess that the **first reason** is the maturity of the frameworks. Spair is younger and has fewer features than Yew. Spair router does not have the same functionality as Yew router. As Spair become more mature in the future, it may produce bigger binary bundle.

Any other reason? The original implementation in Yew has many components. Spair only use one component and split the code using spair::Render. Maybe the number of components affect the size?

So, I modified the two implementations a bit so that the Yew version use some functions instead of components, and the Spair version use components as pages. And here are sizes of `.wasm` files:

| Implemenation                  | Size of `.wasm` | Components (*)                                                         |
| ------------------------------ | --------------- | ---------------------------------------------------------------------- |
| [rustmart-yew-example]         | 477.7kB         | App, Navbar, Home, ProductDetail, RouterAnchor, ProductCard, AtcButton |
| rustmart-yew-fewer-components  | 440.7kB         | App, Home, ProductDetail, RouterAnchor (**)                            |
| rustmart-spair                 | 173.6kB         | App                                                                    |
| rustmart-spair-with-components | 187.2kB         | App, Home, ProductDetail                                               |

*Compliled with Rust 1.46.0*

(*) *Please note that `yew::Component` and `spair::Component` are NOT the same.*

(**) *`Navbar` and `ProductCard` were converted to functions. I am not a user of Yew, so I miserably failed when converting `AtcButton` component to a function, so I just inlined it as an element `<button>...</button>` where the button is required.*

You can see that the difference of Yew's implementations is about 37kB, just by replace 3 components by alternative code. But please note that when moving away from components, you lose the ability to stop the render process (with component, you can return `false` in `Component::change` to tell that Yew should not rerender the component's view)

Using Spair, with 2 more components, 13.6kB was added to the `.wasm` file. (Similar to Yew's implementations, the two implementations in Spair work in different ways internally, but not different visually).

Conclusion? Either you use Yew or Spair, if your primary concern is the size of `.wasm` file, then you may want to avoid using components where possible.

[rustmart-yew-example]: https://github.com/sheshbabu/rustmart-yew-example
