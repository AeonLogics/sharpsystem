use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="home-page">
            <div class="grain-bg"></div>

            // Dynamic Background Elements
            <div class="gradient-orb orb-1"></div>
            <div class="gradient-orb orb-2"></div>
            <div class="gradient-orb orb-3"></div>

            <section class="hero container relative z-10">
                <div class="hero-content center fade-in">
                    <span class="badge hero-badge scale-in">"Phase 2: The Future of Ops"</span>
                    <h1 class="hero-title tracking-tight">
                        "Architecting the" <br/>
                        <span class="gradient-text glow-text">"Intelligent Workflow"</span>
                    </h1>
                    <p class="hero-subtitle text-balance">
                        "A high-performance system engineered with Rust, designed for scale, and optimized for human-speed interaction."
                    </p>
                    <div class="hero-cta flex flex-row gap-6 justify-center">
                        <a href="/register" class="btn btn-primary btn-lg glow-primary px-10 py-5 text-lg font-bold">"Initialize System"</a>
                        <a href="#features" class="btn btn-ghost btn-lg glass-btn px-10 py-5 text-lg">"Explore Architecture"</a>
                    </div>
                </div>

                <div class="hero-visual mt-20 scale-in-up">
                    <div class="glass-card-premium main-preview-card">
                        <div class="preview-header flex items-center justify-between px-8 py-5 bg-black/30 backdrop-blur-xl border-b border-white/5">
                             <div class="dots flex gap-2.5">
                                 <div class="size-2.5 rounded-full bg-red-500/60 shadow-[0_0_10px_rgba(239,68,68,0.2)]"></div>
                                 <div class="size-2.5 rounded-full bg-yellow-500/60 shadow-[0_0_10px_rgba(234,179,8,0.2)]"></div>
                                 <div class="size-2.5 rounded-full bg-green-500/60 shadow-[0_0_10px_rgba(34,197,94,0.2)]"></div>
                             </div>
                             <div class="preview-url bg-white/5 px-6 py-2 rounded-full text-[11px] font-mono text-purple-200/40 border border-white/5 tracking-wider">
                                "sharp-sys://dashboard/overview.v2"
                             </div>
                             <div class="preview-actions flex gap-1.5 opacity-30">
                                 <div class="size-1 bg-white rounded-full"></div>
                                 <div class="size-1 bg-white rounded-full"></div>
                                 <div class="size-1 bg-white rounded-full"></div>
                             </div>
                        </div>
                        <div class="preview-body p-8 lg:p-16">
                            <div class="preview-layout grid grid-cols-[240px_1fr] gap-12">
                                <aside class="preview-sidebar flex flex-col gap-8">
                                    <div class="flex items-center gap-3">
                                        <div class="size-8 rounded-lg bg-purple-500/20 flex items-center justify-center text-xs text-purple-400">"S"</div>
                                        <div class="h-2.5 w-24 bg-purple-500/20 rounded-full"></div>
                                    </div>
                                    <div class="flex flex-col gap-4">
                                        <div class="h-1.5 w-full bg-white/5 rounded-full"></div>
                                        <div class="h-1.5 w-4/5 bg-white/5 rounded-full"></div>
                                        <div class="h-1.5 w-5/6 bg-white/5 rounded-full"></div>
                                        <div class="h-1.5 w-3/4 bg-white/5 rounded-full"></div>
                                    </div>
                                    <div class="mt-auto flex flex-col gap-4">
                                        <div class="h-12 w-full bg-white/5 rounded-xl border border-white/5"></div>
                                        <div class="flex items-center gap-3 px-2">
                                            <div class="size-6 bg-white/10 rounded-full"></div>
                                            <div class="h-1.5 w-16 bg-white/5 rounded-full"></div>
                                        </div>
                                    </div>
                                </aside>
                                <div class="preview-content">
                                    <header class="flex justify-between items-end mb-12">
                                        <div class="flex flex-col gap-3">
                                            <h4 class="text-xs font-mono text-purple-400/80 uppercase tracking-widest">"System Pulse"</h4>
                                            <div class="h-6 w-56 bg-white/10 rounded-lg"></div>
                                        </div>
                                        <div class="flex gap-2">
                                            <div class="size-10 bg-white/5 rounded-xl border border-white/5"></div>
                                            <div class="size-10 bg-purple-500/20 rounded-xl border border-purple-500/20"></div>
                                        </div>
                                    </header>
                                    <div class="grid grid-cols-3 gap-6">
                                        <div class="h-40 bg-white/5 rounded-2xl border border-white/5 p-6 flex flex-col justify-between">
                                            <div class="size-8 bg-white/5 rounded-lg"></div>
                                            <div class="h-2 w-16 bg-white/10 rounded-full"></div>
                                        </div>
                                        <div class="h-40 bg-white/5 rounded-2xl border border-white/5 p-6 flex flex-col justify-between">
                                            <div class="size-8 bg-white/5 rounded-lg"></div>
                                            <div class="h-2 w-16 bg-white/10 rounded-full"></div>
                                        </div>
                                        <div class="h-40 bg-purple-500/5 rounded-2xl border border-purple-500/10 relative overflow-hidden p-6 flex flex-col justify-between">
                                            <div class="absolute inset-0 bg-gradient-to-br from-purple-500/10 to-transparent"></div>
                                            <div class="size-8 bg-purple-500/20 rounded-lg"></div>
                                            <div class="h-2 w-16 bg-purple-500/20 rounded-full"></div>
                                        </div>
                                    </div>
                                    <div class="mt-8 h-32 w-full bg-white/[0.02] rounded-2xl border border-white/5 border-dashed"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="stats-premium container relative z-20">
                <div class="glass-card-premium stats-card">
                    <div class="stats-inner">
                        <div class="stats-grid">
                            <div class="stat-item">
                                <span class="stat-value text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-400">"0ms"</span>
                                <span class="stat-label">"Core Latency"</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-value">"100%"</span>
                                <span class="stat-label">"Rust Core"</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-value">"256-bit"</span>
                                <span class="stat-label">"Encryption"</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-value">"99.9%"</span>
                                <span class="stat-label">"Sync rate"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section id="features" class="features container py-32">
                <div class="section-header center mb-24 flex flex-col gap-6">
                    <div class="h-px w-24 bg-gradient-to-r from-transparent via-purple-500 to-transparent"></div>
                    <h2 class="text-5xl font-bold tracking-tight">"Redefining Performance"</h2>
                    <p class="text-xl text-slate-400 max-w-2xl leading-relaxed">"The intersection of extreme performance and developer experience, built on a bento-style modular architecture."</p>
                </div>

                <div class="bento-grid">
                    <div class="glass-card-premium bento-item item-large group">
                        <div class="icon-box">"⚡"</div>
                        <div class="content">
                            <h3>"Hyper-V Engine"</h3>
                            <p>"Optimized Rust concurrency layers delivering micro-second latency across global edge points."</p>
                        </div>
                    </div>
                    <div class="glass-card-premium bento-item item-small group">
                        <div class="icon-box">"🛡️"</div>
                        <div class="content">
                            <h3>"Safe-Guard"</h3>
                            <p>"Memory-safe core protocols."</p>
                        </div>
                    </div>
                    <div class="glass-card-premium bento-item item-tall group">
                        <div class="icon-box">"📊"</div>
                        <div class="content">
                            <h3>"Real-time Telemetrics"</h3>
                            <p>"Experience live data streaming with zero buffering. Monitor every system pulse with high-fidelity visualizers and automated alerting systems."</p>
                        </div>
                        <div class="mt-12 h-40 bg-white/5 rounded-xl border border-white/5 overflow-hidden">
                             <div class="h-full w-full bg-gradient-to-t from-purple-500/20 to-transparent"></div>
                        </div>
                    </div>
                    <div class="glass-card-premium bento-item item-medium group">
                        <div class="icon-box">"🌐"</div>
                        <div class="content">
                            <h3>"Global Mesh Network"</h3>
                            <p>"Deploy your operations across a resilient, self-healing mesh architecture that scales automatically with your demand."</p>
                        </div>
                    </div>
                </div>
            </section>

            <section class="cta-premium container py-32">
                <div class="glass-card-premium p-24 text-center relative overflow-hidden group">
                    <div class="absolute inset-0 bg-gradient-to-br from-purple-500/10 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-700"></div>
                    <div class="relative z-10 flex flex-col items-center gap-8">
                        <h2 class="text-6xl font-black mb-2 tracking-tighter">"Ready for the Upgrade?"</h2>
                        <p class="text-2xl text-slate-300 max-w-xl mx-auto font-light">"Join the elite teams building on the Sharp System ecosystem."</p>
                        <div class="flex justify-center mt-4">
                            <a href="/register" class="btn btn-primary btn-xl px-16 py-6 text-xl font-black glow-primary tracking-tight">"INITIALIZE NOW —"</a>
                        </div>
                    </div>
                </div>
            </section>

            <footer class="footer container py-20 border-t border-white/5 mt-20">
                <div class="flex flex-col md:flex-row justify-between items-center gap-12 w-full">
                    <div class="flex flex-col gap-4">
                        <p class="text-lg font-black tracking-widest uppercase">"Sharp // System"</p>
                        <p class="text-sm text-slate-500">"The next generation of high-precision management."</p>
                    </div>
                    <div class="flex gap-12">
                         <div class="flex flex-col gap-4">
                             <h5 class="text-xs font-bold uppercase tracking-widest text-purple-400">"Resources"</h5>
                             <a href="#" class="text-sm text-slate-400 hover:text-white transition-colors">"Documentation"</a>
                             <a href="#" class="text-sm text-slate-400 hover:text-white transition-colors">"API Reference"</a>
                         </div>
                         <div class="flex flex-col gap-4">
                             <h5 class="text-xs font-bold uppercase tracking-widest text-purple-400">"System"</h5>
                             <a href="#" class="text-sm text-slate-400 hover:text-white transition-colors">"Status"</a>
                             <a href="#" class="text-sm text-slate-400 hover:text-white transition-colors">"Changelog"</a>
                         </div>
                    </div>
                    <p class="text-sm text-slate-600 font-mono">"© 2026. B-01 // R-4"</p>
                </div>
            </footer>
        </main>
    }
}
