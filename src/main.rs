use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(r##"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Futuristic Rust Website</title>
            <style>
                :root {
                    --primary: #7928ca;
                    --secondary: #ff0080;
                    --background: #0f0e17;
                    --foreground: #fffffe;
                    --accent: #00eeff;
                    --card-bg: rgba(255, 255, 255, 0.05);
                    --card-border: rgba(255, 255, 255, 0.1);
                    --header-height: 70px;
                }
                
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                }
                
                body {
                    background-color: var(--background);
                    color: var(--foreground);
                    line-height: 1.6;
                    overflow-x: hidden;
                    transition: all 0.3s ease;
                }
                
                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 0 2rem;
                }
                
                /* Header and Navigation */
                header {
                    position: fixed;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: var(--header-height);
                    backdrop-filter: blur(10px);
                    background: rgba(15, 14, 23, 0.8);
                    z-index: 1000;
                    border-bottom: 1px solid var(--card-border);
                }
                
                nav {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    height: 100%;
                }
                
                .logo {
                    font-size: 1.5rem;
                    font-weight: 700;
                    background: linear-gradient(to right, var(--primary), var(--secondary));
                    -webkit-background-clip: text;
                    background-clip: text;
                    color: transparent;
                    display: flex;
                    align-items: center;
                }
                
                .logo::before {
                    content: "⚙️";
                    margin-right: 0.5rem;
                    font-size: 1.8rem;
                }
                
                .nav-links {
                    display: flex;
                    gap: 2rem;
                    list-style: none;
                }
                
                .nav-links a {
                    color: var(--foreground);
                    text-decoration: none;
                    font-weight: 500;
                    position: relative;
                    padding: 0.5rem 0;
                    transition: color 0.3s ease;
                }
                
                .nav-links a::after {
                    content: '';
                    position: absolute;
                    bottom: 0;
                    left: 0;
                    width: 0;
                    height: 2px;
                    background: linear-gradient(to right, var(--primary), var(--secondary));
                    transition: width 0.3s ease;
                }
                
                .nav-links a:hover {
                    color: var(--accent);
                }
                
                .nav-links a:hover::after {
                    width: 100%;
                }
                
                .menu-toggle {
                    display: none;
                    flex-direction: column;
                    justify-content: space-between;
                    width: 30px;
                    height: 21px;
                    cursor: pointer;
                }
                
                .menu-toggle span {
                    display: block;
                    height: 3px;
                    width: 100%;
                    background-color: var(--foreground);
                    border-radius: 3px;
                    transition: all 0.3s ease;
                }
                
                /* Hero Section */
                .hero {
                    height: 100vh;
                    display: flex;
                    align-items: center;
                    position: relative;
                    overflow: hidden;
                    padding-top: var(--header-height);
                }
                
                .hero::before {
                    content: '';
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: 
                        radial-gradient(circle at 20% 30%, rgba(121, 40, 202, 0.3), transparent 30%),
                        radial-gradient(circle at 80% 70%, rgba(255, 0, 128, 0.3), transparent 30%);
                    z-index: -1;
                }
                
                .hero-content {
                    max-width: 700px;
                }
                
                .hero h1 {
                    font-size: 3.5rem;
                    line-height: 1.2;
                    margin-bottom: 1.5rem;
                    background: linear-gradient(to right, var(--primary), var(--secondary));
                    -webkit-background-clip: text;
                    background-clip: text;
                    color: transparent;
                }
                
                .hero p {
                    font-size: 1.2rem;
                    margin-bottom: 2rem;
                    color: rgba(255, 255, 255, 0.8);
                }
                
                .cta-button {
                    display: inline-block;
                    padding: 0.8rem 2rem;
                    background: linear-gradient(to right, var(--primary), var(--secondary));
                    color: white;
                    border: none;
                    border-radius: 30px;
                    font-weight: 600;
                    text-decoration: none;
                    cursor: pointer;
                    transition: transform 0.3s ease, box-shadow 0.3s ease;
                    box-shadow: 0 4px 20px rgba(121, 40, 202, 0.5);
                }
                
                .cta-button:hover {
                    transform: translateY(-3px);
                    box-shadow: 0 6px 25px rgba(121, 40, 202, 0.7);
                }
                
                /* Features Section */
                .features {
                    padding: 6rem 0;
                }
                
                .section-title {
                    text-align: center;
                    margin-bottom: 4rem;
                    font-size: 2.5rem;
                    background: linear-gradient(to right, var(--primary), var(--secondary));
                    -webkit-background-clip: text;
                    background-clip: text;
                    color: transparent;
                }
                
                .cards {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 2rem;
                }
                
                .card {
                    background: var(--card-bg);
                    border: 1px solid var(--card-border);
                    border-radius: 15px;
                    padding: 2rem;
                    transition: transform 0.3s ease, box-shadow 0.3s ease;
                    position: relative;
                    overflow: hidden;
                    backdrop-filter: blur(5px);
                }
                
                .card::before {
                    content: '';
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: linear-gradient(45deg, var(--primary), var(--secondary));
                    opacity: 0;
                    z-index: -1;
                    transition: opacity 0.3s ease;
                }
                
                .card:hover {
                    transform: translateY(-10px);
                    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
                }
                
                .card:hover::before {
                    opacity: 0.1;
                }
                
                .card h3 {
                    font-size: 1.5rem;
                    margin-bottom: 1rem;
                    color: var(--accent);
                }
                
                .card-icon {
                    font-size: 2.5rem;
                    margin-bottom: 1.5rem;
                    display: inline-block;
                }
                
                /* About Section */
                .about {
                    padding: 6rem 0;
                    background: rgba(15, 14, 23, 0.5);
                    position: relative;
                }
                
                .about::before {
                    content: '';
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: 
                        radial-gradient(circle at 80% 20%, rgba(121, 40, 202, 0.2), transparent 30%),
                        radial-gradient(circle at 20% 80%, rgba(255, 0, 128, 0.2), transparent 30%);
                    z-index: -1;
                }
                
                .about-content {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 4rem;
                    align-items: center;
                }
                
                .about-text h2 {
                    font-size: 2.5rem;
                    margin-bottom: 1.5rem;
                    background: linear-gradient(to right, var(--primary), var(--secondary));
                    -webkit-background-clip: text;
                    background-clip: text;
                    color: transparent;
                }
                
                .about-image {
                    position: relative;
                    height: 400px;
                    border-radius: 15px;
                    overflow: hidden;
                    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
                }
                
                .about-image::before {
                    content: '';
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: linear-gradient(45deg, var(--primary), var(--secondary));
                    opacity: 0.7;
                    z-index: 1;
                }
                
                .about-image::after {
                    content: '{ Rust }';
                    position: absolute;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    font-size: 3rem;
                    font-weight: 700;
                    color: var(--foreground);
                    z-index: 2;
                }
                
                /* Footer */
                footer {
                    padding: 3rem 0;
                    background: rgba(10, 10, 15, 0.8);
                    border-top: 1px solid var(--card-border);
                }
                
                .footer-content {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }
                
                .footer-links {
                    display: flex;
                    gap: 2rem;
                    list-style: none;
                }
                
                .footer-links a {
                    color: var(--foreground);
                    text-decoration: none;
                    transition: color 0.3s ease;
                }
                
                .footer-links a:hover {
                    color: var(--accent);
                }
                
                .copyright {
                    color: rgba(255, 255, 255, 0.6);
                }
                
                /* Theme Toggle */
                .theme-toggle {
                    position: fixed;
                    bottom: 2rem;
                    right: 2rem;
                    width: 50px;
                    height: 50px;
                    border-radius: 50%;
                    background: linear-gradient(to right, var(--primary), var(--secondary));
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    cursor: pointer;
                    z-index: 100;
                    box-shadow: 0 4px 20px rgba(121, 40, 202, 0.5);
                    transition: transform 0.3s ease;
                }
                
                .theme-toggle:hover {
                    transform: scale(1.1);
                }
                
                .theme-toggle i {
                    font-size: 1.5rem;
                    color: white;
                }
                
                /* Light Theme */
                body.light-theme {
                    --background: #f0f0f0;
                    --foreground: #0f0e17;
                    --card-bg: rgba(0, 0, 0, 0.03);
                    --card-border: rgba(0, 0, 0, 0.1);
                }
                
                body.light-theme header {
                    background: rgba(240, 240, 240, 0.8);
                }
                
                /* Responsive Design */
                @media (max-width: 768px) {
                    .menu-toggle {
                        display: flex;
                    }
                    
                    .nav-links {
                        position: fixed;
                        top: var(--header-height);
                        left: 0;
                        width: 100%;
                        flex-direction: column;
                        background: var(--background);
                        padding: 2rem;
                        gap: 1.5rem;
                        transform: translateY(-100%);
                        opacity: 0;
                        transition: transform 0.3s ease, opacity 0.3s ease;
                        border-bottom: 1px solid var(--card-border);
                    }
                    
                    .nav-links.active {
                        transform: translateY(0);
                        opacity: 1;
                    }
                    
                    .hero h1 {
                        font-size: 2.5rem;
                    }
                    
                    .about-content {
                        grid-template-columns: 1fr;
                    }
                    
                    .footer-content {
                        flex-direction: column;
                        gap: 2rem;
                    }
                }
            </style>
        </head>
        <body>
            <!-- Header & Navigation -->
            <header>
                <div class="container">
                    <nav>
                        <div class="logo">RustFuture</div>
                        <div class="menu-toggle" id="menuToggle">
                            <span></span>
                            <span></span>
                            <span></span>
                        </div>
                        <ul class="nav-links" id="navLinks">
                            <li><a href="#home">Home</a></li>
                            <li><a href="#features">Features</a></li>
                            <li><a href="#about">About</a></li>
                            <li><a href="#contact">Contact</a></li>
                        </ul>
                    </nav>
                </div>
            </header>
            
            <!-- Hero Section -->
            <section class="hero" id="home">
                <div class="container">
                    <div class="hero-content">
                        <h1>The Future of Web Development with Rust</h1>
                        <p>Experience blazing fast performance, guaranteed memory safety, and thread safety in a single file web application.</p>
                        <a href="#features" class="cta-button">Explore Features</a>
                    </div>
                </div>
            </section>
            
            <!-- Features Section -->
            <section class="features" id="features">
                <div class="container">
                    <h2 class="section-title">Powerful Features</h2>
                    <div class="cards">
                        <div class="card">
                            <span class="card-icon">⚡</span>
                            <h3>Blazing Fast</h3>
                            <p>Rust's zero-cost abstractions and compile-time optimizations ensure your web applications run at maximum speed.</p>
                        </div>
                        <div class="card">
                            <span class="card-icon">🔒</span>
                            <h3>Memory Safe</h3>
                            <p>Rust's ownership system guarantees memory safety without a garbage collector, eliminating entire classes of bugs.</p>
                        </div>
                        <div class="card">
                            <span class="card-icon">🧵</span>
                            <h3>Concurrent</h3>
                            <p>Build highly concurrent applications with Rust's fearless concurrency features and thread safety guarantees.</p>
                        </div>
                    </div>
                </div>
            </section>
            
            <!-- About Section -->
            <section class="about" id="about">
                <div class="container">
                    <div class="about-content">
                        <div class="about-text">
                            <h2>Why Choose Rust?</h2>
                            <p>Rust combines low-level performance with high-level ergonomics, making it perfect for web servers, systems programming, and more.</p>
                            <p>With a growing ecosystem and passionate community, Rust continues to evolve as one of the most loved programming languages.</p>
                            <p>This entire website is served from a single Rust file, demonstrating the power and flexibility of Rust for web development.</p>
                        </div>
                        <div class="about-image"></div>
                    </div>
                </div>
            </section>
            
            <!-- Footer -->
            <footer id="contact">
                <div class="container">
                    <div class="footer-content">
                        <div class="copyright">© 2023 RustFuture. All rights reserved.</div>
                        <ul class="footer-links">
                            <li><a href="#">GitHub</a></li>
                            <li><a href="#">Twitter</a></li>
                            <li><a href="#">Discord</a></li>
                        </ul>
                    </div>
                </div>
            </footer>
            
            <!-- Theme Toggle -->
            <div class="theme-toggle" id="themeToggle">
                <i>🌓</i>
            </div>
            
            <!-- JavaScript -->
            <script>
                // Mobile Menu Toggle
                const menuToggle = document.getElementById('menuToggle');
                const navLinks = document.getElementById('navLinks');
                
                menuToggle.addEventListener('click', () => {
                    navLinks.classList.toggle('active');
                    
                    // Animate hamburger to X
                    const spans = menuToggle.querySelectorAll('span');
                    spans[0].style.transform = spans[0].style.transform === 'rotate(45deg) translate(5px, 5px)' ? '' : 'rotate(45deg) translate(5px, 5px)';
                    spans[1].style.opacity = spans[1].style.opacity === '0' ? '1' : '0';
                    spans[2].style.transform = spans[2].style.transform === 'rotate(-45deg) translate(7px, -6px)' ? '' : 'rotate(-45deg) translate(7px, -6px)';
                });
                
                // Smooth Scrolling for Anchor Links
                document.querySelectorAll('a[href^="#"]').forEach(anchor => {
                    anchor.addEventListener('click', function(e) {
                        e.preventDefault();
                        
                        // Close mobile menu if open
                        navLinks.classList.remove('active');
                        
                        // Reset hamburger icon
                        const spans = menuToggle.querySelectorAll('span');
                        spans[0].style.transform = '';
                        spans[1].style.opacity = '1';
                        spans[2].style.transform = '';
                        
                        // Scroll to target
                        document.querySelector(this.getAttribute('href')).scrollIntoView({
                            behavior: 'smooth'
                        });
                    });
                });
                
                // Theme Toggle
                const themeToggle = document.getElementById('themeToggle');
                const body = document.body;
                
                themeToggle.addEventListener('click', () => {
                    body.classList.toggle('light-theme');
                    
                    // Save theme preference
                    const isDarkMode = !body.classList.contains('light-theme');
                    localStorage.setItem('darkMode', isDarkMode);
                });
                
                // Check for saved theme preference
                const savedDarkMode = localStorage.getItem('darkMode');
                if (savedDarkMode === 'false') {
                    body.classList.add('light-theme');
                }
                
                // Intersection Observer for Animations
                const cards = document.querySelectorAll('.card');
                
                const observer = new IntersectionObserver((entries) => {
                    entries.forEach(entry => {
                        if (entry.isIntersecting) {
                            entry.target.style.opacity = '1';
                            entry.target.style.transform = 'translateY(0)';
                        }
                    });
                }, { threshold: 0.1 });
                
                cards.forEach(card => {
                    card.style.opacity = '0';
                    card.style.transform = 'translateY(20px)';
                    card.style.transition = 'opacity 0.5s ease, transform 0.5s ease';
                    observer.observe(card);
                });
                
                // Header Scroll Effect
                window.addEventListener('scroll', () => {
                    const header = document.querySelector('header');
                    if (window.scrollY > 50) {
                        header.style.boxShadow = '0 5px 20px rgba(0, 0, 0, 0.1)';
                    } else {
                        header.style.boxShadow = 'none';
                    }
                });
            </script>
        </body>
        </html>
    "##)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

