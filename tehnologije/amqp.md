# AMQP

AMQP (Advanced Message Queue Protocol) je protokol aplikacionog nivoa orijentisan na poruke. On je binarni protokol i oslanja se na TCP. Može da podrži autentifikaciju/enkripciju baziranu na SASL/TSL-u. Ovo pruža svojstvo neporecivosti porukama koje se šalju preko AMQP protokola. Osnovna jedinica prenosa podataka je *frame*. AMQP može da garantuje za podatke at-most-once, at-least-once ili exactly-once osnosno da će podaci stići jednom ili ni jednom, makar jednom ili tačno jednom. AMQP nije implementiran kao API već kao žični protokol, tako da se komunikacija može odvijati između klijenata koji znaju da interpretiraju i kreiraju te bajtove koji se prenose preko mreže.

## RabbitMQ

RabbitMQ je broker poruka otvorenog koda. Inicjalno implementira AMQP protokkol ali se pomoću plagina može proširiti i na STOMP i MQTT protokole. Implementiran je u Erlang programskom jeziku i na Open Telecom Platform radnom okviru za klasterovanje.

<img width="1872" height="867" alt="image" src="https://github.com/user-attachments/assets/5cf0113a-06e7-4a9f-8b2a-da922da5a96b" />

## Arhitektura

<img width="1203" height="764" alt="image" src="https://github.com/user-attachments/assets/08cab803-8d2b-44b3-84fc-50d92ad34201" />
<img width="1189" height="804" alt="image" src="https://github.com/user-attachments/assets/698d9e98-a0ab-433c-9f17-cd948945344b" />




## Reference
https://en.wikipedia.org/wiki/Advanced_Message_Queuing_Protocol

https://en.wikipedia.org/wiki/RabbitMQ

https://www.infoq.com/articles/AMQP-RabbitMQ/

https://www.rabbitmq.com/resources/google-tech-talk-final/alexis-google-rabbitmq-talk.pdf

https://www.rabbitmq.com/resources/google-tech-talk-final/google
