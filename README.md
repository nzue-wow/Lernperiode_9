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

- [x] (GET methode) Frontend eine Request an Backend schicken lassen und die Daten(für Taschenrechner) holen
- [x] (PUT methode) Die Daten im Backend manipulieren/verändern
- [x] (DELETE methode) Daten im Backend löschen
- [x] (POST methode) Daten erstellen im Backend
- [x] Ich werde alle Arbeitspakete in Bezug auf die Weiterführung des Taschenrechners erstellen.

Heute habe ich alle http methoden in meinem Code realisiert. Jetzt kann man mit Set(POST) eine Zahl ins Backend hinzufügen. Mit add(PUT) kann man die Daten im Backend verändern. Mit Reset(DELETE) kann man Jetzt den Taschenrechner wieder auf 0 stellen. Und mit GET refresh(GET) kann man die aktuelle zahl aus dem backend holen. Der Taschenrechner jetzt testet bis jetzt nur ob alle Methoden funktionieren. Also bis jetzt addiert er nur die zahlen die eingegeben werden.

# 13.03.2026
- [x] Alle operatoren zum Taschenrechner hinzufügen(+, -, *, /)
- [ ] Man soll mehrere Zahlen können eingeben
- [x] Man soll ergebnisse Speichern können
- [x] Das Design des Rechners funktional und visuell schön machen

 Heute habe ich den Taschenrechner Prototypen erweitert. Ich habe die Operatoren eingebaut und man kann auch die Zahlen Speichern. Was nochnicht geht ist mehrere Zahlen gleichzeitig einzugeben. Man kann jetzt ein Zahl eingeben und dann den Operator dazu auswählen. Sobald das gemacht wird, wird eine request ans backend geschickt und jenachdem welcher Operator es ist, ist es ein anderer Rechnungsweg. Dann habe ich auch das Aussehen geändert das es jetzt mehr nach einem Tschenrechner aussieht. 


# 20.03.2026
- [x] Man soll die Zahlen gleichzeitg aufschreiben können also: 1 + 1. Dann soll man enter drücken und dann soll das ergebnis kommen
- [x] Die Zahlen in einer Datenbak speichern(nicht nur im arbeitsspeicher)
- [x] Eine CSS datei machen damit der code besser aussieht
- [ ] Neuer Operator hinzufügen `^2`

Heute Habe ich versucht mein Programm mit SMMS SQL zu verbinden. Leider hat das nicht funktioniert wegen des Logins. Dann habe ich auf SQLlite gewechselt das dann funktioniert hat. Jetzt werden die Ergebnisse gespeichert. Jetzt funktioniert der Taschenrechner auch richtig weil. Man mehrere Zahlen eingeben und ausrechen. Ausserdem habe ich das Design noch komplett geändert so das es für alles einen Button hat damit man es nicht selber schreiben muss.

# 27.03.2026
- [ ] Den ganzen Code anschauen, meine doku anschauen(schauen was alles schief gegangen ist im Projekt) und damit ein Tutorial machen
- [ ] Den Code ausbessern das man ich auch versteht(Html in eigenes dokument, kommentäre)
