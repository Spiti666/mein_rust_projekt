Single-File Rust Webserver – README (English)
Overview
This project demonstrates a futuristic-looking website served by a single Rust file using the Actix-Web framework. The layout and design are handled by HTML/CSS, while Actix-Web provides a fast, asynchronous web server in Rust.

Features 
Modern Design & Styling

Dark/Light mode using CSS variables
Smooth transitions, hover effects, and responsive layout
Interactive hamburger menu and animated elements
Rust-Powered Backend

Uses Actix-Web for high performance and scalability
Memory safety and multi-threading features from Rust
All-in-one file deployment for easy maintenance
Front-End Interactivity

Smooth scrolling, theme switching, intersection observer for card animations
Minimal JavaScript for advanced effects
Getting Started
Install Rust and Cargo
Follow the official guide: https://www.rust-lang.org/tools/install
Clone or Copy the Project
bash
Kopieren
Bearbeiten
git clone https://github.com/your-username/single-file-rust-webserver.git
cd single-file-rust-webserver
Run the Server
bash
Kopieren
Bearbeiten
cargo run
The console will display something like:
arduino
Kopieren
Bearbeiten
Server running at http://127.0.0.1:8080
View the Site
Open your browser and go to http://127.0.0.1:8080.
Code Explanation
Actix-Web Imports: get, HttpServer, HttpResponse, Responder, etc. are used to create routes and serve responses.
Route Definition: The #[get("/")] attribute binds the index() function to the root ("/") path.
HTML String: A complete HTML/CSS/JavaScript page is served directly from a Rust string.
Main Function: Sets up and runs the Actix server on port 8080.
Folder Structure (Suggestion)
Although this demo is contained in a single file, a more typical setup might look like this:

css
Kopieren
Bearbeiten
.
├── Cargo.toml
├── src
│   └── main.rs
├── static
│   ├── index.html
│   ├── css
│   └── js
└── ...
static/: Contains HTML, CSS, JavaScript, images, etc.
main.rs: The Rust server code.
Cargo.toml: Defines the project’s metadata and dependencies.
Customization
Separate Files: Move HTML, CSS, and JS into their own files for easier maintenance.
Template Engine: If you need dynamic data, consider using Tera or a similar Rust templating engine.
Database Integration: Add SQLx or Diesel for persistent data.
Docker: Containerize this application for deployment in Docker-based environments.
License
Use and modify this code freely based on your needs. If you plan to distribute it, adding a standard open-source license (e.g., MIT, Apache 2.0) is recommended.

Single-File Rust Webserver – README (Deutsch)
Überblick
Dieses Projekt präsentiert eine futuristisch anmutende Webseite, die von einer einzigen Rust-Datei mit dem Actix-Web-Framework bereitgestellt wird. HTML/CSS kümmern sich um das Layout und Design, während Actix-Web einen schnellen, asynchronen Webserver in Rust zur Verfügung stellt.

Features
Modernes Design & Styling

Dunkel-/Hellmodus per CSS-Variablen
Sanfte Übergänge, Hover-Effekte und responsives Layout
Interaktives Hamburger-Menü und animierte Elemente
Rust-Backend

Nutzt Actix-Web für hohe Leistung und Skalierbarkeit
Speicher- und Thread-Sicherheit durch Rust
Alles in einer Datei für einfache Wartung
Interaktivität im Frontend

Sanftes Scrollen, Theme-Wechsel und Intersection Observer für Kartenanimationen
Minimaler JavaScript-Einsatz für erweiterte Effekte
Erste Schritte
Rust und Cargo installieren
Befolge die offizielle Anleitung: https://www.rust-lang.org/tools/install
Projekt klonen oder kopieren
bash
Kopieren
Bearbeiten
git clone https://github.com/dein-benutzername/single-file-rust-webserver.git
cd single-file-rust-webserver
Server starten
bash
Kopieren
Bearbeiten
cargo run
Die Konsole zeigt dann etwa:
arduino
Kopieren
Bearbeiten
Server running at http://127.0.0.1:8080
Webseite aufrufen
Öffne deinen Browser und rufe http://127.0.0.1:8080 auf.
Code-Erklärung
Actix-Web Imports: get, HttpServer, HttpResponse, Responder etc. dienen zur Erstellung von Routen und zum Ausliefern von Responses.
Routen-Definition: Das Attribut #[get("/")] verknüpft die Funktion index() mit dem Root-Pfad "/".
HTML-String: Eine vollständige HTML-/CSS-/JavaScript-Seite wird direkt aus einer Rust-String-Konstante ausgegeben.
Main-Funktion: Richtet den Actix-Server auf Port 8080 ein und startet ihn.
Verzeichnisstruktur (Empfehlung)
Zwar enthält dieses Beispiel alles in einer Datei, doch typischerweise wählt man:

css
Kopieren
Bearbeiten
.
├── Cargo.toml
├── src
│   └── main.rs
├── static
│   ├── index.html
│   ├── css
│   └── js
└── ...
static/: Enthält HTML, CSS, JavaScript, Bilder usw.
main.rs: Rust-Server-Code.
Cargo.toml: Projekt-Metadaten und Abhängigkeiten.
Anpassungsmöglichkeiten
Dateien trennen: Lagere HTML, CSS und JavaScript in separate Dateien aus, um die Wartung zu erleichtern.
Template Engine: Für dynamische Daten (z. B. Tera).
Datenbankintegration: Füge SQLx oder Diesel hinzu, um persistente Daten zu speichern.
Docker: Containerisiere die Anwendung für Deployments in Docker-Umgebungen.
