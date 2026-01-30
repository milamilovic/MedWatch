# AWS SNS

Amazon Simple Notification Service (SNS) je AWS-ov upravljani servis za razmenu poruka po publish/subscribe (pub/sub) modelu. To znači da izdavači (publishers) šalju poruke na teme (topics), a pretplatnici (subscribers) te poruke primaju. SNS tema se možete posmatrati kao komunikacioni kanal, to jest neko objavljuje poruke na taj kanal, a svi koji su pretplaćeni budu odmah obavešteni. Prednost ovakvog sistema je u tome što izdavači ne moraju da znaju ko prima njihove poruke, a pretplatnici ne moraju da znaju ko ih šalje. 

SNS podržava i komunikaciju između aplikacija (Application-to-Application, A2A) i komunikaciju između aplikacije i osobe (Application-to-Person, A2P). Na taj način je omogućena velika fleksibilnost prilikom isporuke poruka. Može se kpristiti više protokola poput HTTP(s) endpointa, SMS poruka, elektronske pošte ili push notifikacija u okviru aplikacije.

<img width="700" height="700" alt="image" src="https://github.com/user-attachments/assets/62d2e2a6-ab1b-46c5-8167-4814ef1f6a2a" />

Glavne komponente od kojih se sastoji Simple Notification Service su:
1. Publisher
2. Topic
3. Subscriber(s)

## Topic
Za SNS Topic može da se kaže da je centralna komponenta u Amazon SNS-u jer se koristi kao komunikacioni kanal između izdavača i svih pretplatnika. Izdavači ga koriste da bi objavili poruke, dok pretplatnici definišu na koji način žele da primaju poruke. Prilikom kreiranja topic-a, može da se izabere između Standard i FIFO (First In First Out) tipova topic-a. Kod Standard topic-a, SNS ne garantuje da će poruke stići pretplatnicima u istom redosledu u kome su objavljene i nije onemogućeno dupliranje poruka. Zbog toga se koristi za isporuku notifikacija gde redosled nije bitan faktor, ali je važno da poruka stigne. S druge strane, FIFO topic se koristi u aplikacijama gde je veoma važno očuvanje redosleda poruka i jednokratna isporuka. Iz razloga što FIFO topic-i imaju manji protok poruka, obično se koriste u kritičnim sistemima poput transakcija na primer. SNS topic-i podržavaju višestruke protokole za isporuku, to jest poruke poslate na jedan topic mogu stići i do HTTPs endpoint-a, AWS Lambda, email-a, SMS text-ova i tako dalje. Po default-u, pretplatnik dobija sve poruke koje se objave na neki topic, osim ako ne definiše (filter policy) politiku kojom će se filtrirati poruke koje mu stižu. 

## Publisher 
Izdavač ili publisher predstavlja komponentu ili aplikaciju koja kreira i objavljuje poruke na topic. Uloga izdavača je samo da prosledi SNS-u poruku bez potrebe da zna ko su krajnji primaoci te poruke jer SNS vrši distribuciju svih poruka. Izdavač može biti bilo kakav servis ili aplikacija koja može da koristi AWS sDK, AWS CLI ili REST API za slanje poruka. Kada publisher pošalje poruku na određeni topic, SNS tu poruku prihvata, vrši osnovnu validaciju formata i zatim je automatski prosleđuje svim pretplatnicima koji su registrovani za taj topic. Publisher ima mogućnost da postavi i dodatne atribute uz poruku na osnovu kojih pretplatnici mogu da filtritaju poruke koje im stižu. 

## Subscriber(s)
Ovo je komponenta koja prima poruke poslate na određeni topic. Pretplatnik može biti različite vrste. U slučaju Application-to-Application (A2A), pretplatnik može biti HTTP/HTTPS endpoint, Amazon SQS queue, AWS Lambda funkcija ili Kinesis Data Firehose. To omogućava asinhrono procesiranje poruka i integraciju između servisa bez potrebe za povezivanjem sa izdavačem. U Application-to-Person (A2P), pretplatnik može biti mobilni uređaj sa SMS porukama, email adresa ili mobilna aplikacija za push notifikacije i na taj način je omogućena direktna komunikacija sa krajnjim korisnicima. SNS pretplatnici se registruju na topic i automatski primaju sve poruke objavljene na tom topic-u. Mogu se koristiti i mehanizmi kao što su retry isporuka, filtriranje poruka po atributima i upravljanje redosledom slanja. 

### Application‑to‑Application (A2A) subscribers
A2A subscribers predstavljaju sisteme i servise kojima se poruke isporučuju direktno na druge aplikacione komponente umesto na krajnje korisnike, čime je omogućena integracija mikroservisa ili drugih AWS servisa. 
Najčešće podržani A2A pretplatnici su:
 - HTTP/HTTPS endpoints (SNS šalje POST zahteve ka web serverima ili mikroservisima)
 - Amazon SQS queues (funkcionišu kao privremeni redovi za obrađivanje poruka asinhrono)
 - AWS Lambda funkcije (prosleđuje im se poruka kako bi se izvršio neki kod)
 - Amazon Kinesis Data Firehose (slanje događaja u skladišta za analitiku poput S3, Redshift ili OpenSearch)

### Application‑to‑Person (A2P) subscribers
A2P subscribers omogućavaju da se poruke koje su generisane u nekom sistemu isporuče krajnjim korisnicima, umesto drugim servisima. 
Najvažniji A2P pretplatnici su:
 - SMS (Short Message Service) (omogućava slanje mobilnih tekstualnih poruka direktno na telefonske brojeve korisnika širom sveta)
 - Email adrese (obaveštenja se šalju u formi elektronske pošte korisnicima)
 - Mobile push notifications (SNS isporučuje poruke mobilnim aplikacijama (iOS, Android) uz pomoć posebnih servisa kao što su APNs ili FCM)

### SMS (mobile text) kao A2P pretplatnik
SMS (Short Message Service) je jedan od Application to Person pretplatnika koji omogućava sistemima da šalju tekstualne poruke krajnjim korisnicima putem njihovih mobilnih telefona. Komunikacija je na globalno dostupna bez potrebe za sopstvenom infrastrukturom za SMS, jer SNS sam upravlja konekcijama sa mobilnim operatorima i mehanizmima isporuke. Mehanizmi isporuke predstavljaju automatske ponovne pokušaje u slučaju neuspeha i upravljanje troškovima i ograničenjima isporuke po regionima. SMS poruke se mogu koristiti za hitna obaveštenja, verifikacione kodove, upozorenja o događajima ili bilo koju drugu vrstu tekstualne komunikacije sa krajnjim korisnicima. Pretplatnik mora biti verifikovan da bi primao obaveštenja sa željenog topic-a.

## Reference
- https://docs.aws.amazon.com/sns/latest/dg/welcome.html
- https://www.datacamp.com/tutorial/aws-sns
- https://aws.amazon.com/tr/sns/features/
- https://medium.com/@joudwawad/aws-sns-deep-dive-6cc9cefbb9bb
