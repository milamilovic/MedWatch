# AWS SNS

Amazon Simple Notification Service (SNS) je AWS-ov upravljani servis za razmenu poruka po publish/subscribe (pub/sub) modelu. To znači da izdavači (publishers) šalju poruke na teme (topics), a pretplatnici (subscribers) te poruke primaju. SNS tema se možete posmatrati kao komunikacioni kanal, to jest neko objavljuje poruke na taj kanal, a svi koji su pretplaćeni budu odmah obavešteni. Prednost ovakvog sistema je u tome što izdavači ne moraju da znaju ko prima njihove poruke, a pretplatnici ne moraju da znaju ko ih šalje. 

SNS podržava i komunikaciju između aplikacija (Application-to-Application, A2A) i komunikaciju između aplikacije i osobe (Application-to-Person, A2P). Na taj način je omogućena velika fleksibilnost prilikom isporuke poruka. Može se kpristiti više protokola poput HTTP(s) endpointa, SMS poruka, elektronske pošte ili push notifikacija u okviru aplikacije.

<img width="700" height="700" alt="image" src="https://github.com/user-attachments/assets/62d2e2a6-ab1b-46c5-8167-4814ef1f6a2a" />


## Reference
- https://docs.aws.amazon.com/sns/latest/dg/welcome.html
- https://www.datacamp.com/tutorial/aws-sns
