# MQTT

MQTT (Message Queuing Telemetry Transport) je mrezni protokol za razmenu poruka koji je baziran na publish-subscribe principu. Dizajniran je za konekciju sa fizički udaljenim uređajima sa ograničenim resursima kao što su IoT uređaji. MQTT je binarni protokol i oslanja se na TCP/IP za pouzdan transport poruka. Podržava tri nivoa kvaliteta usluge (QoS):
- *at-most-once* - poruka se šalje samo jednom bez potvrde o prijemu, može se izgubiti
- *at-least-once* - poruka se isporučuje najmanje jednom i mogući su duplikati
- *exactly-once* - poruka se isporučuje tačno jednom kroz četvorosmerni handshake
MQTT sam po sebi šalje kredencijale u plain text-u ali može da se koristi TLS/SSL za enkripciju. Dokazano je da je bezbednost ovog protokola narušena 2020. godine kada su naučnici izveli Slow Denial of Service.

## Arhitektura


## Reference
https://en.wikipedia.org/wiki/MQTT

https://en.wikipedia.org/wiki/Slow_DoS_attack
