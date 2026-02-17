# AMQP izvestaj

Analiziran je sistem za zdravstveni monitoring gde IoT uređaji šalju medicinske podatke pacijenata putem AMQP message broker-a, konkretno RabbitMQ-a. Kao bezbednosna pretnja identifikovana je kategorija napda koji ciljaju dostupnost (availability) sistema prema CIA trijadi. 

## Stablo napada
<img width="4208" height="812" alt="image" src="https://github.com/user-attachments/assets/abca2d73-efee-44cb-8ac6-eb9b7e5b4399" />

### Praktično realizovan napad
Napad eksploatiše kombinaciju podrazumevanih kredencijala (guest/guest), odsustva ograničenja veličine poruka i odsustva ograničenja u broju redova. Sistem koji je implementiran se sastoji iz tri komponente koje su kontejnerizovane i organizovane pomoću docker compose-a.

- #### Implementacija ranjive aplikacije
    Ranjiv sistem se sastoji iz RabbbitMQ message brokera i aplikacije koja je implementirana u Rust programskom jeziku. RabbitMQ je pokrenut sa podrazumevanim kredencijalima za prijavu, i bez ograničenja u bilo kakvom korišćenju resursa. Aplikacija šalje simulirane podatke u json formatu bez validacija u broju ili dužini poruka.

- #### Implementacija napada
    Napad se izvodi pokretanjem aplikacije koja je implementirana u Rust-u koja izvodi Resource Exhaustion napad na tri načina. Prvi je slanje poruka od 10MB, zatim kreira 1000 malicioznih redova i nakon toga šalje 10000 poruka čime izvodi message flooding.

- #### Implementacija mitigovane aplikacije
    Mitigovan sistem se kao i ranjiv sastoji iz RabbitMQ message brokera i aplikacije u Rust-u, ali je RabbitMQ konfigurisan tako da su podrazumevani kredencijali za prijavu zamenjeni sa nalogom sa jakom šifrom i sa ograničenjima i u broju poruka i u njihovoj dužini. Sama aplikacija za razliku od ranjive sadrži validaciju veličine poruka, rate limiting i ograničenjima u vezi queue-ova.

- #### Video demonstracija
    TODO

### Napadi koji nisu realizovani
Preostala četiri napada iz stabla nisu implementirani u praktičnom delu, ali su teorijski relevantni za potpunu bezbednosnu analizu sistema. 

- #### DoS HTTP API-ja
    TODO

- #### DoS consumer aplikacije
    TODO

- #### Loše kreiran AMQP
    TODO

- #### Konfiguraciona manipulacija (policy injection)
    TODO
