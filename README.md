# Lernperiode_9

Technologie auswählen

Da ich in der letzten Lernperiode Rust als Frontend benutzt habe, wollte ich nun Rust im Backend-Bereich kennenlernen.

Ich habe mir die drei Technologien ausgewählt:

- Axum: Axum ist ein modernes Web-Framework für Rust. Damit kann man HTTP-Server und APIs bauen. Es hat mich angesprochen, weil es aktuell und gut strukturiert ist.

- Actix-Web: Actix-Web ist ebenfalls ein Rust-Webframework. Es gilt als sehr performant, wirkt aber etwas komplexer.

- FastAPI: FastAPI ist ein Python-Framework für APIs. Es wäre einfacher gewesen, aber ich wollte bei Rust bleiben und mein Wissen erweitern.

Ich habe mich für Axum entschieden, weil ich weiterhin mit Rust arbeiten möchte, aber diesmal im Backend-Bereich. So erweitere ich meine Kenntnisse in Rust und lerne zusätzlich, wie man eine REST-API entwickelt.

# 20.02.2026

Heute habe ich ein sehr einfaches Frontend mit Yew erstellt.
Das Frontend zeigt nur einen Titel und einen Text an, um zu testen, ob Yew im Browser läuft.

Zusätzlich habe ich ein Backend mit Axum programmiert.
Ich habe einen einfachen HTTP-Server erstellt und eine Route gemacht, die „Hello from Rust backend!“ zurückgibt.
Den Server habe ich lokal gestartet und im Browser getestet.

Damit habe ich ein simples Frontend und ein laufendes Backend als Vorbereitung für den Tracer Bullet Prototype aufgebaut.

# 27.02.2026

- [x] Das Backendstarten und im Browser testen, schauen ob der Server läuft
- [x] Ich starte das Frontend und prüfe im Browser ob die Seite angezeigt wird(cargo run funktioniert hier nicht ich bekomme immer ein error)- rausfinden ob ich überhaupt das richtige mache und wieso es ein error gibt
- [ ] Backend route ändern damit ich eine JSON-Antwort bekomme
- [x] Die Errors beim ausführen rausfinden

Heute habe ich mein backend ein bisschen verändert, also einfacher gemacht. Dann habe ich das Backend mit dem Frontend verbunden, indem das frontend ein Request an das backend schickt und somit meine Nachricht die ich vorher ins backend geschrieben habe vom Backend zum frontend geschickt wird. Das heisst mein tracer Bullet Prototype habe ich gemacht. Es gibt jetzt auch kein error mehr wenn ich das frontend starte. Ich musste dafür das trunk paket runterladen damit ich das Frontend starten konnte. Das Frontend startet man mit `trunk serve` und das backend mit `cargo run`.

# 06.03.2026

- [ ] (GET methode) Frontend eine Request an Backend schicken lassen und die Daten(für Taschenrechner) holen
- [ ] (PUT methode) Die Daten im Backend manipulieren/verändern
- [ ] (DELETE methode) Daten im Backend löschen
- [ ] (POST methode) Daten erstellen im Backend
- [ ] Ich werde alle Arbeitspakete in Bezug auf die Weiterführung des Taschenrechners erstellen.
