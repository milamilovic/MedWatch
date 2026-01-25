# AMQP

AMQP (Advanced Message Queue Protocol) je protokol aplikacionog nivoa orijentisan na poruke osnosno prenosi poruke u celosti a ne strimove bajtova. On je binarni protokol i oslanja se na TCP. Može da podrži autentifikaciju kroz SASL (Simple Authentication and Security Layer) i enkripciju kroz TLS/SSL (Transport Layer Security/Secure Socket Layer). Osnovna jedinica prenosa podataka je *frame*. AMQP podržava tri tipa garancije isporuke: 
- *at-most-once* odnosno da će poruka biti isporučena jednom ili ni jednom
- *at-least-once* gde će poruka biti isporučena makar jednom
- *exactly-once* odnosno da će podaci stići tačno jednom.

AMQP nije implementiran kao API već kao žični protokol, tako da se komunikacija može odvijati između klijenata koji znaju da interpretiraju i kreiraju te bajtove koji se prenose preko mreže.

<br/>
<img width="691" height="297" alt="image" src="https://github.com/user-attachments/assets/d0677cba-7b1e-45d7-afee-59cc9997de19" />
<br/><br/>

Logička arhitektura AMQP servera je standardizovana da bi se garantovala interoperabilnost između različitih implementacija i zasniva se na razdvajanju rutiranja poruka od njihovog skladištenja. Proizvođači (publishers) šalju poruke u exchange, koji na osnovu pravila rutiranja i bindinga prosleđuje poruke odgovarajućim queue-ovima. Queue čuva poruke (u memoriji ili na disku) i isporučuje ih potrošačima (consumers) kada su spremni da ih obrade. 


## RabbitMQ

RabbitMQ je broker poruka otvorenog koda. Inicjalno implementira AMQP protokol ali se pomoću plagina može proširiti i na STOMP i MQTT protokole. Implementiran je u Erlang programskom jeziku. 


## Arhitektura

<br/>
<img width="1512" height="705" alt="mermaid-diagram-2026-01-20-204042" src="https://github.com/user-attachments/assets/129f5d84-5c17-4f40-91ed-48ceb4536c82" />
<br/><br/>

Ključne komponente RabbitMQ-a su:

### Producer
Producer ili publisher je aplikacija koja šalje poruke. Ista aplikacija može u isto vreme biti i producer i consumer tj može i da šalje i da prima poruke. Poruke se šalju u exchange-ove. Poruke se zatim rutiraju u odgovarajući queue na osnovu ključa koji producer šalje. Implemenitran je *acknowledgement mechanism* odnosno mehanizam u kom broker potvrdjuje producer-u da je poruka primljena. Producer može da postavi i metadata odnosno dodatne atribute za poruke kao što su timestamp, prioritet, persistence mode i slično.

### Exchange
Prima poruke od producera i rutira ih u queue-ove prema pravilima. Postoje četiri tipa exchange-a:
- **Direct** - rutira poruke na osnovu tačnog poklapanja ključa za rutiranje
- **Fanout** - broadcast svim povezanim queue-ovima, ignoriše ključ za rutiranje
- **Topic** - rutira poruke na osnovu pattern matching-a ključa za rutiranje
- **Headers** - rutira na osnovu header atributa umesto ključa za rutiranje
Obavezni atributi exchange-a su trajnost koja opisuje da li će exchange preživeti restart i auto-deletion koji govori da li će se exchange obrisati kada se oslobodi poslednji binding.

### Queue
Bafer koji čuva poruke u po jednom FIFO redu za svakog consumer-a dok on ne bude spreman da ih obradi odnosno uređena kolekcija poruka. Svaki consumer ima svoj queue. Queue može čuvati poruke na disku ili u memoriji u zavisnosti od persistence mode-a. Obavezni atributi queue-a su ime, durability (da li preživljava restart brokera), exclusive (da li će biti obrisan nakon završetka jedne konekcije) kao i auto-delete koji govori da li će queue koji je imao makar jednog consumer-a biti obrisan kada se on unsubscribe-uje.

### Binding
Veza između exchange-a i queue-a sa pravilom (pattern) koje određuje koje poruke će biti rutirane u taj queue. Binding sadrži source name koji je u stvari ime exchange-a, destination name tj ime ciljanog queue-a ili exchange-a, destination type i još opcionih argumenata poput header-a. Jedan queue može imati više bindinga, i jedan exchange može biti povezan sa više queue-ova. 

### Consumer
Consumer je aplikacija koja prima i obrađuje poruke iz queue-a. Consumer subscribe-uje na neki queue i automatski mu se dostavljaju poruke iz njega. Prilikom registracije mogu da odaberu manual ili automatic delivery type odnosno da li šalje potvrdu prijema ili ne. Da bi se consumer otkazao mora biti poznat njegov identifikator odnosno tag i kada se on otkaže poruke odmah prestaju da mu se prosleđuju. 

### Management UI
todo

### Broker
Implementacija RabbitMQ Brokera se sastoji od generičkog tcp soket-a koji služi kao ulazna/izlazna tačka za komunikaciju. Ona dalje komunicira sa Reader komponentom koja čita bajtove koji stižu sa mreže i šalje ih framing channel-u koji ih pretvara u AMQP frejmove. Channel komponenta izvršava AMQP komande, updavlja transakcijama i proverava permisije. Writer komponenta prima frejmove od channel-a i pretvara ih u bajtove za slanje preko mreže. RabbitMQ server takođe sadrži i mnesia-u što je distribuirana baza podataka u Erlang-u koja čuva definicije queue-ova i exchange-ova, naloge korisnika i njihove permisije. Poslednja komponenta servera je amqqueue tj red čekanja poruka koje još uvek nisu obrađene.

<br/>
<img width="623" height="532" alt="image" src="https://github.com/user-attachments/assets/e2ebdfc6-8706-4eaa-a057-a3bcfa4830dd" />
<br/><br/>

### HTTP API
todo

### Connection
todo

### Channel
todo

### Virtual host
todo

## Reference
https://en.wikipedia.org/wiki/Advanced_Message_Queuing_Protocol

https://en.wikipedia.org/wiki/RabbitMQ

https://www.infoq.com/articles/AMQP-RabbitMQ/

https://www.rabbitmq.com/resources/google-tech-talk-final/alexis-google-rabbitmq-talk.pdf

https://www.rabbitmq.com/resources/google-tech-talk-final/google

https://www.rabbitmq.com/resources/specs/amqp0-9-1.pdf

https://www.rabbitmq.com/tutorials/amqp-concepts

https://www.rabbitmq.com/docs/publishers

https://www.rabbitmq.com/docs/exchanges

https://www.rabbitmq.com/docs/queues

https://www.rabbitmq.com/docs/consumers
