use yew::prelude::*;

#[function_component(About)]
pub fn about_function_component() -> Html {
    return html! {
        <section>
            <div class="container mt-1 p-4 bg-dark">
                 <div class="row justify-content-md-center text-center mb-4">
                    <p class="text-white fs-2">{"IlluviAnalytics is a passion project of"}</p>
                    <div class="col-4 animate__animated animate__fadeInLeft animate__fast">
                      <img src="/img/enigo.jpg" class="img-fluid border border-2 border-light rounded-4 shadow-gradient" width="250" height="250" alt="Enigo"/>
                      <p class="text-white fs-4">{"Enigo"}</p>
                    </div>
                    <div class="col-4 animate__animated animate__fadeInRight animate__fast">
                      <img src="/img/angel.jpg" class="img-fluid border border-2 border-light rounded-4 shadow-gradient" width="250" height="250" alt="Angel"/>
                      <p class="text-white fs-4">{"Angel"}</p>
                    </div>
                 </div>
                 <div class="row justify-content-md-center text-center mb-4 g-1">
                    <p class="text-white fs-4">{"Following API providers are used to fetch the data"}</p>
                    <div class="col-md-5">
                        <ul class="list-group">
                          <li class="list-group-item bg-pink"><a href="https://docs.x.immutable.com" target="_blank" class="text-decoration-none fs-5">{"ImmutableX"}</a></li>
                          <li class="list-group-item bg-pink"><a href="https://www.coingecko.com" target="_blank" class="text-decoration-none fs-5">{"CoinGecko"}</a></li>
                          <li class="list-group-item bg-pink"><a href="https://docs.etherscan.io" target="_blank" class="text-decoration-none fs-5">{"Etherscan"}</a></li>
                        </ul>
                    </div>
                    <p class="text-white fs-6 mt-3">{"All prices on the site are displayed in buyer (aka taker) denomination
                                which includes royalty, ecosystem and protocol fees (where applicable)"}</p>
                 </div>
            </div>
            <div class="container-fluid p-4 bg-gray">
                <div class="container">
                     <div class="row justify-content-md-center text-center mb-4">
                        <p class="text-white fs-2">{"Roadmap"}</p>
                        <p class="text-white fs-4 mb-0">{"This is the very first version of the website in which we wanted to concentrate on utility"}</p>
                        <p class="text-white fs-4">{"But there are great plans for more features"}</p>
                        <div class="col-md-5">
                            <ul class="list-group">
                              <li class="list-group-item bg-pink"><p class="text-white fs-5 m-1">{"Fetching deposits and withdrawals data"}</p></li>
                              <li class="list-group-item bg-pink"><p class="text-white fs-5 m-1">{"Fetching today's conversion rate data"}</p></li>
                              <li class="list-group-item bg-pink"><p class="text-white fs-5 m-1">{"Loading data for all Illuvium projects"}</p></li>
                              <li class="list-group-item bg-pink"><p class="text-white fs-5 m-1">{"Adding new page for Illuvium Universe monetary data"}</p></li>
                              <li class="list-group-item bg-pink"><p class="text-white fs-5 m-1">{"Adding new page for wallet stats"}</p></li>
                              <li class="list-group-item bg-pink"><p class="text-white fs-5 m-1">{"Introducing 'find similar assets' functionality"}</p></li>
                              <li class="list-group-item bg-pink"><p class="text-white fs-5 m-1">{"Assets Explore UI overhaul"}</p></li>
                              <li class="list-group-item bg-pink"><p class="text-white fs-5 m-1">{"General UI overhaul (because we can make it better)"}</p></li>
                            </ul>
                        </div>
                        <p class="text-white fs-5 mt-4 mb-1">{"Have feedback to share? That's awesome, send it over!"}</p>
                        <a href="mailto:info@pudding.pro" class="text-decoration-none fs-5">{"info@pudding.pro"}</a>
                     </div>
                 </div>
            </div>
            <div class="container p-4 bg-dark">
                <div class="row justify-content-md-center text-center mt-4">
                    <p class="text-muted mb-0">
                        <small>{"Logo and portrait images generated by "}
                            <a href="https://www.bing.com/images/create" target="_blank" class="text-decoration-none">{"Bing Image Creator"}</a>
                        </small>
                    </p>
                </div>
            </div>
        </section>
    };
}