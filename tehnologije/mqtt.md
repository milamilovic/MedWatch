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

### Broker
Mosquitto broker je centralna komponenta koja prima konekcije od klijenata preko TCP socketa, obično na portu 1883 za neenkriptovane koekcije a na portu 8883 za TLS konekcije. Broker održava internu strukturu podataka koja mapira topic-e na listu pretplaćenih klijenata i kada primi PUBLISH poruku, pretražuje svoju subscription mapu i prosleđuje poruku svim klijentima koji su pretplaćeni na odgovarajući topic pattern. Mosquitto upravlja sesijom-om za svaku konekciju klijenta, retained porukama za svaki topic, in-flight porukama za QoS 1 i QoS 2, kao i client subscriptions sa njihovim QoS nivoima. Broker podržava više metoda autentifikacije uključujući username/password kroz password fajl, TLS Client Certificates pomoću X.509 sertifikata i plugin sistem za custom autentifikaciju. Autorizacija se konfiguriše kroz ACL (Access Control List) fajlove koji definišu koja konekcija može da pristupa kojim topic-ovima i sa kojim permisijama. Mosquitto može da čuva podatke na disku za trajnost u slučaju restarta, uključujući retained poruke, session state za trajne sesije i in-flight poruke koje nisu još isporučene. Broker takođe podržava bridging funkcionalnost koja omogućava povezivanje više brokera u distribuiranu mrežu i prosleđivanje topic-a između njih.

### Subscribers
mosquitto_sub je klijent iz komandne linije za pretplaćivanje na topic-e i primanje poruka sa MQTT brokera i on može da se pretplati na više topic-a istovremeno. Alat podržava wildcard karaktere u topic pattern-ima gde + zamenjuje jedan nivo u hijerarhiji a # zamenjuje sve nivoe od te tačke naniže. mosquitto_sub može da prikaže poruke u različitim formatima uključujući verbose mode sa -v opcijom koji prikazuje topic uz poruku, što je korisno kada se prati više topic-ova. Opcija -F omogućava custom format string koji može prikazati topic, payload, QoS, retain flag, timestamp i druge detalje poruke. Alat podržava filtriranje poruka po topic-u i može da koristi regular expressions za naprednije pattern matching sa -T opcijom. mosquitto_sub može da skladišti primljene poruke u fajlove pomoću --output opcije ili da ih prosleđuje na stdout za dalju obradu drugim alatima.

mosquitto_rr je klijent iz komandne linije koji implementira request-response obrazac čime omogućava sinhronu komunikaciju preko MQTT protokola. Ovaj alat radi tako što objavljuje poruku na određeni topic i zatim čeka odgovor na response topic-u. Alat je posebno koristan za implementaciju RPC (Remote Procedure Call) obrasca preko MQTT-a gde klijent šalje zahtev i očekuje odgovor od servera. mosquitto_rr je koristan za testiranje MQTT servisa koji implementiraju request-response pattern bez potrebe za pisanjem klijentskog koda.

### Publisher
mosquitto_pub je klijenti iz komandne linije za objavljivanje poruka na MQTT broker. On omogućava slanje pojedinačnih poruka ili čitanje podataka sa standardnog ulaza i podržava sva tri QoS nivoa i može da postavlja retain flag kako bi broker sačuvao poslednju poruku za određeni topic. mosquitto_pub može da učitava poruke iz fajlova koristeći -f opciju ili da pročita jednu liniju sa stdin-a pomoću -l opcije, što omogućava pipe-ovanje podataka iz drugih programa. Alat podržava TLS/SSL enkripciju kroz sertifikate i privatne ključeve, kao i pre-shared key (PSK) autentifikaciju. Korisnik može da konfiguriše Last Will Testament poruku koja će biti poslata ako klijent neočekivano izgubi konekciju sa brokerom. mosquitto_pub omogućava slanje null poruka pomoću -n opcije što je korisno za brisanje retained poruka sa određenog topic-a.

## Reference
https://en.wikipedia.org/wiki/MQTT

https://en.wikipedia.org/wiki/Slow_DoS_attack

https://mosquitto.org/documentation/

https://mosquitto.org/man/mosquitto_rr-1.html

https://mosquitto.org/man/mosquitto_sub-1.html

https://mosquitto.org/man/mosquitto_pub-1.html

