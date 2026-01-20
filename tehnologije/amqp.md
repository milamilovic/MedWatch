# AMQP

AMQP (Advanced Message Queue Protocol) je protokol aplikacionog nivoa orijentisan na poruke osnosno prenosi poruke u celosti a ne strimove bajtova. On je binarni protokol i oslanja se na TCP. Može da podrži autentifikaciju kroy SASL (Simple Authentication and Security Layer) i enkripciju kroz TLS/SSL (Transport Layer Security/Secure Socket Layer). Ovo pruža svojstvo neporecivosti porukama koje se šalju preko AMQP protokola. Osnovna jedinica prenosa podataka je *frame*. AMQP podržava tri tipa garancije isporuke: *at-most-once* odnosno da će poruka biti isporučena jednom ili ni jednom, *at-least-once* gde će poruka biti isporučena makar jednom, kao i *exactly-once* odnosno da će podaci stići tačno jednom. AMQP nije implementiran kao API već kao žični protokol, tako da se komunikacija može odvijati između klijenata koji znaju da interpretiraju i kreiraju te bajtove koji se prenose preko mreže.

## RabbitMQ

RabbitMQ je broker poruka otvorenog koda. Inicjalno implementira AMQP protokkol ali se pomoću plagina može proširiti i na STOMP i MQTT protokole. Implementiran je u Erlang programskom jeziku. Njegove ključne komponente su:

- Producer - aplikacija koja šalje poruke i one se ne šalju direktno u queue već u exchange
- Exchange - prima poruke od producera i rutira ih u queue-ove prema pravilima
- Queue - bafer koji čuva poruke u FIFO redu dok consumer ne bude spreman da ih obradi
- Binding - veza između exchange-a i queue-a sa pravilom (pattern) koje određuje koje poruke će biti rutirane u taj queue
- Consumer - aplikacija koja prima i obrađuje poruke iz queue-a


<img width="1872" height="867" alt="image" src="https://github.com/user-attachments/assets/5cf0113a-06e7-4a9f-8b2a-da922da5a96b" />

## Arhitektura

<img width="1203" height="764" alt="image" src="https://github.com/user-attachments/assets/08cab803-8d2b-44b3-84fc-50d92ad34201" />

<img width="1189" height="804" alt="image" src="https://github.com/user-attachments/assets/698d9e98-a0ab-433c-9f17-cd948945344b" />

Prva komponenta RabbitMQ servera je generički tcp soket koji služi kao ulazna/izlazna tačka za komunikaciju. Ona dalje komunicira sa Reader komponentom koja čita bajtove koji stižu sa mreže i šalje ih framing channel-u koji ih pretvara u AMQP frejmove. Channel komponenta izvršava AMQP komande, updavlja transakcijama i proverava permisije. Writer komponenta prima frejmove od channel-a i pretvara ih u bajtove za slanje preko mreže. RabbitMQ server takođe sadrži i mnesia-u što je distribuirana baza podataka u Erlang-u koja čuva definicije queue-ova i exchange-ova, naloge korisnika i njihove permisije. Poslednja komponenta servera je amqqueue tj red čekanja poruka koje još uvek nisu obrađene.  


## Reference
https://en.wikipedia.org/wiki/Advanced_Message_Queuing_Protocol

https://en.wikipedia.org/wiki/RabbitMQ

https://www.infoq.com/articles/AMQP-RabbitMQ/

https://www.rabbitmq.com/resources/google-tech-talk-final/alexis-google-rabbitmq-talk.pdf

https://www.rabbitmq.com/resources/google-tech-talk-final/google
