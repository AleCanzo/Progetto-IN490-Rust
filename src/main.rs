use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
//use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::Timestamp;



//let mut counter :i32 = 0;
//let mut counter1 :i32 = 0;
//let mut counter2 :i32 = 0;
//let mut counter3 :i32 = 0;

struct Handler;



static mut COUNTER :i32 = 0;
static mut COUNTER1 :i32 = 0;
static mut COUNTER2 :i32 = 0;
static mut COUNTER3 :i32 = 0;

#[async_trait]
  


    impl EventHandler for Handler {
        
        async fn message(&self, ctx: Context, msg: Message) {
            unsafe{  
            
            //let mut counter :i32 = 0;
            //let mut counter1 :i32 = 0;
            //let mut counter2 :i32 = 0;
            //let mut counter3 :i32 = 0;

            if msg.content == "Come nasce il genere metal?" {
                //println!("ciao");
                //println!("{}", COUNTER);
                COUNTER=COUNTER+1;
                COUNTER1=COUNTER1+1;
                //println!("{}", COUNTER);
                if let Err(why) = msg.channel_id.say(&ctx.http, "Le origini dell'heavy metal possono essere ritrovate all'interno del movimento british blues degli anni sessanta, in particolare tra quelle band che trovarono difficile riadattare il proprio sound a quello del classico blues americano. Durante questo periodo, le ritmiche divennero più essenziali, e gli strumenti elettrici amplificati cominciarono ad assumere maggiore importanza, soprattutto grazie alle innovazioni dei primi gruppi hard rock dei metà anni sessanta come the Kinks, the Who, Jimi Hendrix, Cream, the Jeff Beck Group, Yardbirds e Eric Clapton, i quali anticiparono il genere sia nel sound che nell'attitudine.").await{
                    println!("Error sending message: {:?}", why);
                    //println!("ciao");
                }
                
            }
            if msg.content == "Quali sono i gruppi più rappresentativi del metal?" {
                COUNTER=COUNTER+1;
                COUNTER1=COUNTER1+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "I gruppi più rappresentativi del metal sono Judas Priest, Black Sabbath, Ozzy Osbourne, Metallica e Motorhead.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Quanta gente ascolta metal nel mondo?" {
                COUNTER=COUNTER+1;
                COUNTER1=COUNTER1+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Come mostrano i dati di Spotify il metal è un business globale, e i metallari sono ovunque").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Cosa caratterizza la musica metal?" {
                COUNTER=COUNTER+1;
                COUNTER1=COUNTER1+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Il metal è caratterizzato da ritmi fortemente aggressivi e da un suono potente, ottenuto attraverso l'enfatizzazione dell'amplificazione e della distorsione delle chitarre, dei bassi, e, talvolta, persino delle voci.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Quanti sottogeneri del rock ci sono?" {
                COUNTER=COUNTER+1;
                COUNTER1=COUNTER1+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "I sottoggeneri del rock sono veramente tanti! Consulta questo link: https://it.wikipedia.org/wiki/Categoria:Generi_musicali_rock").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Come nasce il rock?" {
                COUNTER=COUNTER+1;
                COUNTER1=COUNTER1+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Le basi della musica rock sono nel rock and roll, che ha avuto origine negli Stati Uniti tra la fine degli anni Quaranta e l'inizio degli anni Cinquanta e si è rapidamente diffuso in gran parte del resto del mondo. Le sue origini immediate risiedono in una fusione di vari generi musicali neri dell'epoca, inclusi rhythm and blues e musica gospel, con la musica country. Il rock and roll ha rappresentato non solamente un nuovo genere musicale, ma anche un vero e proprio fenomeno sociale.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Quali sono i gruppi più importanti nel rock moderno?" {
                COUNTER=COUNTER+1;
                COUNTER1=COUNTER1+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Secondo il sito R3M, i gruppi più importanti adesso sono i Larkin Poe, Hands Off Gretel, Jinjer, Tyler Bryant & The Shakedown e In This Moment.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Quali strumenti vengono usati nel rock?" {
                COUNTER=COUNTER+1;
                COUNTER1=COUNTER1+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Le sonorità del rock sono caratterizzate prevalentemente dall'utilizzo di strumenti elettrici, in particolare la chitarra elettrica,[4] che in genere viene accompagnata da una sezione ritmica costituita da basso elettrico[5] e batteria.[6] Frequente negli anni sessanta fu la presenza dell'organo elettronico, come il Vox Continental e l'Hammond. Dagli anni settanta in poi, sempre più frequentemente, hanno iniziato a fare la loro comparsa anche i sintetizzatori.[7] La strumentazione rock di base è stata derivata dalla strumentazione di base delle band blues (chitarra solista di spicco, secondo strumento a corda, basso e batteria).").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Come nasce il blues?" {
                COUNTER=COUNTER+1;
                COUNTER2=COUNTER2+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Come avviene per altre forme di musica popolare, le origini del blues, in quanto poco documentate e oscure, sono oggetto di tante discussioni.In particolare non c'è una precisa data di nascita per questo genere musicale: la traccia più antica di una forma musicale simile al blues è il racconto che, nel 1901, fece un archeologo del Mississippi, descrivendo il canto di lavoratori neri che sembra avere affinità melodiche e liriche con il blues di oggi. Non è, dunque, possibile stabilire con esattezza una data che segni l'origine del genere, tuttavia un anno fondamentale fu il 1865, anno dell'abolizione della schiavitù negli Stati Uniti d'America: ottenuta la libertà, numerosi ex schiavi-musicisti iniziarono a portare la loro musica fuori dalle piantagioni e, nel giro di qualche decennio, questo genere fu noto ai più fino a giungere alle prime attestazioni che ci sono pervenute.").await {
                    println!("Error sending message: {:?}", why);
                    
                }
            }
            if msg.content == "Quali sono gli artisti più importanti del blues?" {
                COUNTER=COUNTER+1;
                COUNTER2=COUNTER2+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Gli artisti blues più importanti sono B.B. King, Eric Clapton, Robert Johnson, Muddy Waters e altri ancora... Guarda qui: https://www.debaser.it/main/classifica_generale.aspx?idGen=36&idAgg=1&idAnn=0&idOgg=2").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Come nasce il jazz?" {
                COUNTER=COUNTER+1;
                COUNTER2=COUNTER2+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "La musica che originariamente sarebbe stata chiamata, con termine di origine incerta jazz, nasce quasi certamente a New Orleans all'inizio del XX secolo. Il musicista cui è attribuito il titolo di 'padre del jazz', Buddy Bolden, è attivo a New Orleans nel 1904. Nel 1906 il pianista Jelly Roll Morton compose il brano King Porter Stomp, che fu uno dei primi brani jazz a godere di vasta notorietà, e negli anni seguenti a New Orleans furono attive molte formazioni jazz: tra le più importanti, quella capeggiata dal cornettista Joe 'King' Oliver. La parola 'jazz' venne stampata da un quotidiano, per la prima volta, nel 1913.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Com'è formato un gruppo jazz?" {
                COUNTER=COUNTER+1;
                COUNTER2=COUNTER2+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "La formazione jazzistica moderna tipica è costituita da un gruppo musicale di dimensioni limitate. La combinazione più frequente è il quartetto, quasi invariabilmente costituito da una sezione ritmica composta da batteria, basso o contrabbasso, pianoforte e da uno strumento solista, generalmente un sassofono o una tromba.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Quali sono i compositori di musica classica più famosi?" {
                COUNTER=COUNTER+1;
                COUNTER2=COUNTER2+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Secondo il sito IBS.it, i 'magnifici 8' della musica classica sono Johann Sebastian Bach, Ludwig van Beethoven, Fryderyk Chopin, Wolfgang Amadeus Mozart, Pëtr Il'ič Čajkovskij, Claude Debussy, Richard Wagner e Giuseppe Verdi.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Cos'è la musica classica?" {
                COUNTER=COUNTER+1;
                COUNTER2=COUNTER2+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Col termine musica classica ci si riferisce alla musica colta, sacra e profana, composta o avente radici nel contesto della cultura occidentale. Essa abbraccia approssimativamente un arco di tempo che comincia dall'XI secolo e si estende fino al XX secolo[1] o, a seconda delle convenzioni, fino all'età contemporanea. Tale periodo include, in particolare, il periodo caratterizzato dallo sviluppo e impiego prevalente dell'armonia tonale, codificata tra il XVII e il XIX secolo[2]. In contesti più specializzati il termine 'musica classica' può essere anche riferito, in senso più restrittivo, al periodo musicale detto Classicismo[3], ma nel linguaggio comune l'espressione è intesa nel suo significato più esteso (in opposizione a musica leggera o a musica popolare).").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Quali strumenti sono usati nella musica classica?" {
                COUNTER=COUNTER+1;
                COUNTER2=COUNTER2+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Gli strumenti utilizzati più comunemente nel repertorio della musica classica sono stati in buona parte inventati prima della metà del XIX secolo (spesso molto prima), e hanno sviluppato la loro forma moderna tra XVI e XIX secolo. Gli strumenti più comuni sono presenti nell'orchestra o nella banda, insieme a numerosi altri strumenti ad uso principalmente solistico (come il pianoforte, il clavicembalo e l'organo). L'orchestra sinfonica è la formazione musicale d'insieme più conosciuta al grande pubblico per l'esecuzione della musica classica[12] e comprende le famiglie degli archi, fiati (legni e ottoni) e percussioni. La banda è un altro complesso musicale che spesso esegue musica classica; è composta da strumenti appartenenti alle famiglie dei legni, ottoni e percussioni. Essa ha in genere una varietà più ampia (soprattutto tra gli ottoni) e una maggiore quantità di strumenti a fiato rispetto all'orchestra, ma non ha una sezione di archi.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Perchè è importante la musica classica?" {
                COUNTER=COUNTER+1;
                COUNTER2=COUNTER2+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Teorie neurologiche dimostrano e avvalorano la tesi dell’Effetto Mozart, secondo cui, oltre che per sviluppare il ragionamento spazio-temporale, favorire la concentrazione, ottimizzare i tempi di apprendimento e permettere il rilassamento, ascoltare la musica del grande maestro aiuta a compensare e restituire carenze dovute a danni subiti: su tutto ciò si basa l’odierna musicoterapia che riesce a curare molte patologie.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Come nasce il rap?" {
                COUNTER=COUNTER+1;
                COUNTER3=COUNTER3+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "In origine il rap nasce dalle performance dei dj, che avevano l’abitudine di parlare su una base musicale per coinvolgere il pubblico. È una forma di improvvisazione che si è evoluta nel tempo, e che prevede anche la manipolazione del piatto dei dischi per produrre ritmi e l’uso di campionamenti sonori.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Quali sono i rapper più influenti in Italia?" {
                COUNTER=COUNTER+1;
                COUNTER3=COUNTER3+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "I rapper italiani più influenti sono salmo, Marracash e… https://www.esquire.com/it/cultura/musica/a23560495/rapper-italiani-famosi/").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Cos'è la trap?" {
                COUNTER=COUNTER+1;
                COUNTER3=COUNTER3+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "La trap è un sottogenere musicale dell'hip hop, derivante dal southern hip hop, nato nel Sud degli Stati Uniti e sviluppatosi tra la fine degli anni novanta e l'inizio degli anni 2000. Con il termine trap si può anche indicare il genere di musica EDM nato negli anni 2010 dalla fusione di trap e dubstep: la cosiddetta EDM Trap.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Cosa caratterizza la trap?" {
                COUNTER=COUNTER+1;
                COUNTER3=COUNTER3+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Questa musica, alle origini, era molto legata ad ambienti e tematiche relative a vendita e dipendenza da droghe e da alcool: inizialmente non era un genere vero e proprio, fino ai primi anni del 2000 il termine indicava semplicemente un luogo (le trap house, appunto) ma successivamente comincia a essere utilizzato per indicare la musica legata a quel contesto. Le trap houses erano appartamenti abbandonati e degradati nei sobborghi di Atlanta in cui si spacciavano sostanze stupefacenti, inoltre la parola trapping in slang significa 'spacciare'.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Chi sono i primi 3 classificati su spotify?" {
                COUNTER=COUNTER+1;
                COUNTER3=COUNTER3+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Scoprilo tu: https://open.spotify.com/playlist/37i9dQZEVXbNG2KDcFcKOF?si=4b5cb16a60e24563").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Qual è la definizione di musica pop?" {
                COUNTER=COUNTER+1;
                COUNTER3=COUNTER3+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Nella lingua inglese, i termini popular music e pop music sono spesso usati impropriamente in modo intercambiabile. Il termine popular music è un termine generico che si riferisce alla musica di gradimento generale nell'epoca moderna, mentre la notazione di pop music si riferisce ad uno specifico genere musicale. Secondo Gianni Sibilla, la musica pop «indica un campo più ristretto e definito rispetto a quello di popular music».").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Quando è nata la musica pop?" {
                COUNTER=COUNTER+1;
                COUNTER3=COUNTER3+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "La musica pop moderna nasce nella metà degli anni cinquanta negli Stati Uniti e nel Regno Unito. Se negli anni sessanta in Italia il termine era poco usato in favore del più generico 'musica leggera', sul finire del decennio veniva usato per indicare genericamente tutti quei gruppi che uscivano dalle forme imposte dalla canzonetta, e che vedevano nel Festival di Sanremo il loro motore principale. Venivano quindi inseriti senza distinzione nella definizione di pop gruppi musicali che andavano dalla musica beat al rock psichedelico e al rock progressivo. Solo in seguito il termine 'pop' fu usato con l'accezione con cui ci si riferisce a livello internazionale, con la classificazione e distinzione del rock come specifico genere.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            if msg.content == "Cosa caratterizza la musica pop?" {
                COUNTER=COUNTER+1;
                COUNTER3=COUNTER3+1;
                if let Err(why) = msg.channel_id.say(&ctx.http, "La musica pop, secondo Gianni Sibilla, è «contraddistinta da alcuni aspetti specifici che riguardano il periodo storico di produzione, le forme testuali e linguistiche, gli attori sociali coinvolti, il modo in cui essi costruiscono la propria identità e, soprattutto, il rapporto con i mezzi di comunicazione di massa. In altre parole, la musica pop è un macrogenere musicale contemporaneo che comprende tutti i sottogeneri specifici della canzone popolare sviluppatisi a partire dall'avvento del rock and roll, contraddistinti dalla diffusione intermediale su supporti fonografici e mezzi di comunicazione».").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            
            if COUNTER == 5 {
                COUNTER = 0;
                //println!("ciao");
                if let Err(why) = msg.channel_id.say(&ctx.http, "**Ho capito chi sei!**").await {
                    println!("Error sending message: {:?}", why);
                }
                if COUNTER1 > COUNTER2 && COUNTER1 > COUNTER3 {
                //println!("ciao1");
                    //COUNTER = 0;
                    COUNTER1 = 0;
                    COUNTER2 = 0;
                    COUNTER3 = 0;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Creativo, difficilmente hai cali di autostima e tendi ad essere introverso. Sei un tipo rock!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
                if COUNTER2 > COUNTER1 && COUNTER2 > COUNTER3 {
                    //COUNTER = 0;
                    COUNTER1 = 0;
                    COUNTER2 = 0;
                    COUNTER3 = 0;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Sei creativo, introverso e ti piace tanto riflettere. Che classe! ").await {
                    println!("Error sending message: {:?}", why);
                }
            }
                if COUNTER3 > COUNTER1 && COUNTER3 > COUNTER2 {
                    //COUNTER = 0;
                    COUNTER1 = 0;
                    COUNTER2 = 0;
                    COUNTER3 = 0;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Sei estroverso e socievole. Sei un tipo alla moda!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
                if COUNTER1 == 2 && COUNTER2 == 2 {
                    //COUNTER = 0;
                    COUNTER1 = 0;
                    COUNTER2 = 0;
                    COUNTER3 = 0;
                if let Err(why) = msg.channel_id.say(&ctx.http, "Sei creativo, sicuro di te e pensi fuori dagli schemi!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
                if COUNTER1 == 2 && COUNTER3 == 2 {
                    //COUNTER = 0;
                    COUNTER1 = 0;
                    COUNTER2 = 0;
                    COUNTER3 = 0;
                if let Err(why) = msg.channel_id.say(&ctx.http, "A volte sei introverso, a volte sei estroverso… Ma sappiamo che sei sicuro di te!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
                if COUNTER2 == 2 && COUNTER3 == 2 {
                    //COUNTER = 0;
                    COUNTER1 = 0;
                    COUNTER2 = 0;
                    COUNTER3 = 0;
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Un occhio al passato ed uno al presente… Ci sai fare!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            
            }
            
            if msg.content != "!presentati" {
                return;
            }

            let msg = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content("")
                        .embed(|e| {
                            e.title("BOT per progetto IN490")
                                .description("Sono qui per scoprire che tipo di persona sei attraverso i tuoi gusti musicali. Mettimi alla prova con 5 di queste domande e ti dirò chi sei!")
                                //.image("https://raw.githubusercontent.com/serenity-rs/serenity/current/examples/e09_create_message_builder/ferris_eyes.png")
                                .fields(vec![
                                    ("Domande:", "1. Come nasce il genere metal? \n
                                        2. Quali sono i gruppi più rappresentativi del metal? \n
                                        3. Quanta gente ascolta metal nel mondo? \n
                                        4. Cosa caratterizza la musica metal? \n
                                        5. Quanti sottogeneri del rock ci sono? \n
                                        6. Come nasce il rock? \n
                                        7. Quali sono i gruppi più importanti nel rock moderno? \n
                                        8. Quali strumenti vengono usati nel rock? \n
                                        9. Come nasce il blues? \n
                                        10. Quali sono gli artisti più importanti del blues? \n
                                        11. Come nasce il jazz? \n
                                        12. Com’è formato un gruppo jazz? \n
                                       
                                        ", true), 
                                    (".", "13. Quali sono i compositori di musica classica più famosi? \n
                                        14. Cos’è la musica classica? \n
                                        15. Quali strumenti sono usati nella musica classica? \n
                                        16. Perché è importante la musica classica? \n
                                        17. Come nasce il rap? \n
                                        18. Quali sono i rapper più influenti in Italia? \n
                                        19. Cos’è la trap? \n
                                        20. Cosa caratterizza la trap? \n
                                        21. Chi sono i primi 3 classificati su spotify? \n
                                        22. Qual è la definizione di musica pop?\n 
                                        23. Quando è nata la musica pop?\n
                                        24. Cosa caratterizza la musica pop?
                                        ", true),
                                ])
                                //.field("Altri comandi:", "!clear (cancello gli ultimi 10 messaggi)", false)
                                //.footer(|f| f.text("Autore: Alessandro Canzoneri"), f.image("https://raw.githubusercontent.com/serenity-rs/serenity/current/examples/e09_create_message_builder/ferris_eyes.png"))
                                .timestamp(Timestamp::now())
                        })
                        //.add_file("./ferris_eyes.png")
                })
                .await;
    
            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            }
        }
    
        

        
    
       
    }
       
}


#[tokio::main]
async fn main() {
    
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    //println!("prova");
    //let mut counter :i32 = 0;
    //let mut counter1 :i32 = 0;
    //let mut counter2 :i32 = 0;
    //let mut counter3 :i32 = 0;
}

