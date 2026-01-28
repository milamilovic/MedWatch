# MQTT

MQTT (Message Queuing Telemetry Transport) je mrezni protokol za razmenu poruka koji je baziran na publish-subscribe principu. Dizajniran je za konekciju sa fizički udaljenim uređajima sa ograničenim resursima kao što su IoT uređaji. MQTT je binarni protokol i oslanja se na TCP/IP za pouzdan transport poruka. Podržava tri nivoa kvaliteta usluge (QoS):
- *at-most-once* - poruka se šalje samo jednom bez potvrde o prijemu, može se izgubiti
- *at-least-once* - poruka se isporučuje najmanje jednom i mogući su duplikati
- *exactly-once* - poruka se isporučuje tačno jednom kroz četvorosmerni handshake
MQTT sam po sebi šalje kredencijale u plain text-u ali može da se koristi TLS/SSL za enkripciju. Dokazano je da je bezbednost ovog protokola narušena 2020. godine kada su naučnici izveli Slow Denial of Service.

<br/>
<img src="https://github.com/user-attachments/assets/1f8d9323-8a1c-468b-a5da-9f1f3ac53822" />
<br/><br/>

MQTT arhitektura se zasniva na broker modelu gde centralni server (broker) posreduje u komunikaciji između klijenata. Klijenti mogu biti publishers (objavljuju poruke), subscribers (pretplaćeni na poruke) ili oboje. Komunikacija se odvija preko topic-a koji su organizovani hijerarhijski. MQTT broker prima sve poruke od publisher-a i prosleđuje ih odgovarajućim subscriber-ima. Broker ne čuva poruke trajno, već samo radi kao posrednik u realnom vremenu. Publisher je klijent koji šalje poruke brokeru. Publisher ne mora da zna ko su subscriber-i. Kada šalje poruku, publisher navede topic, playload, QoS niv i retain flag koji navodi da li broker treba da sačuva poslednju poruku za topic. Publisher može da objavi poruke na više različitih topic-a ili može odmah da zatvori konekciju nakon slanja. Subscriber je klijent koji se pretplaćuje na topic-ove i prima poruke od brokera. On može biti pretplaćen na više topic-a istovremeno i može dinamički dodavati ili uklanjati pretplate tokom aktivne sesije. Topic je UTF-8 string koji broker koristi za rutiranje poruka. Organizovan je hijerarhijski sa / kao separatorom nivoa. 

## Mosquitto

Mosquitto je open source MQTT broker. Razvila ga je Eclipse Foundation i napisan je u programskom jeziku C. Mosquitto je jedan od najpopularnijih MQTT brokera zbog svoje jednostavnosti, performansi i lake konfiguracije.


## Arhitektura


## Reference
https://en.wikipedia.org/wiki/MQTT

https://en.wikipedia.org/wiki/Slow_DoS_attack
