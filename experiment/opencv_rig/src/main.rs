// So I had a fun idea: you know how people hate excessive comments in code?
// What if I wrote a blog post style explainer thingy while I try implementing
// some webcam -> opencv -> web browser viewer idea I just had for this project?
// It seems like a cool way to explain a concept, since the reader can literally
// copy your code and experiment with your ideas wheile reading. I wonder what
// the naysayers would say about my excessive comments then?
//
// Well, probably still not nice things, but for the rest of you here's me
// trying out a new idea literally with the following immediately below me as I
// type:
//
// ```
// fn main() {
//     println!("Hello, world!");
// }
// ```
//
// Hopefully it's interesting! For the rest of you, :s/^\/\/.*$//g


fn main() {
    println!("Hello, world!");

    // So anyways, the idea here is that I'm working on a thing to automate my
    // home garden. During quarantine I tried my hand at growing various fruits
    // using hydroponic gardening (involves growing without bringing
    // nasty-smelling soil indoors), and realized that plants are kinda like
    // biological machines that also make tasty things you can eat! I also
    // realized that growing plants is a huge pain in the ass, and my engineer
    // brain took over as I realized that the fact I was using hydroponics
    // meant that I could theoretically electronically regulate not only the
    // concentration of oxygen, nitrogen and potassium in the water, as well as
    // monitor its PH and any microbeal growth, but since I was growing indoors
    // in a closet anyways I could also theoretically electronically control
    // the humidity, temperature, and light concentration that the plant
    // experiences! I can also measure all of these things, plus the signs that
    // the plant itself visually shows, so I'm thinking that if I can figure out
    // some sort of method(s) to determine plant health, I could build a system
    // to optimally regulate plant health. Who knows, if I could make it cheap
    // and reliably enough maybe it could even be done at scale? Worst case it's
    // just a fun method for remotely tending my garden and an excuse to build a
    // fun electronics project that involves computer vision and stuff, which
    // are areas I want to get better in!
    //
    // Good ideas (hopefully) and all, but if I'm going to make it real I'm
    // thinking I'll need to figure out the following problems:
    //
    // 1.) How am I going to measure what's going on in the environmet.
    // 2.) How am I going to regulate my plant's environment.
    // 3.) How am I going to figure out my plant's health?
    // 4.) How am I going to make the setup cheaper (the sensors are $$$).
    //
    // I also have some far fetched ideas about building a robotic arm thing
    // that can maintain the immediate area of the plant as well as take care of
    // any physical plant maintenance and eventual harvesting. I think I could
    // probably build something like that, but I'd definitely have a lot of
    // problems with along the way. Who knows if I'll even work on this idea
    // long enough, but hey I bought 3 webcams so I can capture visual data
    // over time about my plants on all 3 axes! I figure that'll be useful for
    // both assessing the plant's health as well as maybe helping a robot see
    // it while it works with it.
    //
    // Anyways, I framed the problem like that on purpose--those bullet points
    // map to nice midway points that give me stuff I can actually use to assist
    // me in gardening! Let me rephrase them roughly here to illustrate:
    //
    // 1.) I want a method of remotely checking up on my plants so I can figure
    //     out if I need to do anything before walking in there and testing the
    //     water like a primitive caveman.
    // 2.) I want a method of remotely fixing my plants' environment if the
    //     conditions are off-schedule or the environment is otherwise out of
    //     whack (further caveman-tendancy avoidance).
    // 3.) I want my plants to be healthier and more productive than rule of
    //     thumb grow schedules and tips from SEO-boosted marketing casual
    //     gardening sites will allow.
    //
    // I can easily chop away at these over time and learn stuff along the way!
    // Starting with goal #1 here, as you might have gathered this
    // post/experiment is going to be all about reading frames from the webcams
    // I bought, running them through opencv to possibly do some computer vision
    // stuff with later (I know literally nothing about it but it's typically a
    // common example for AI classes so how hard could it be to recognize
    // patterns in plants?), then output that data to some interface. For this
    // experiment, that means writing some code to try setting up this pipeline!
    //
    // Anywho, time to get started!

    use opencv::videoio::{VideoCapture, CAP_ANY);

    // I did some light research on the opencv thing, and not only came away
    // with indimidating-sounding future topics of study like deep learning (I
    // have some idea of what they involve, but none of the depth of the
    // problem space), but also a cool example that showed that it has some nice
    // built-in capacity for capturing video frames in a manipulable format!
    // This is good, because outputting it somewhere is likely going to involve
    // rendering it into a format that the recieving program can understand.
    // Since I don't actually want to distribute an application for this, that
    // means I'm going to have to make it viewable in a browser! More
    // elaboration on that on that later in this file though, right now we need
    // to capture frames.

    let cameras: Vec<VideoCapture> = Vec::new();

    const MAX_CAMERAS = 10;  // Arbitrary, since I can't find a logical limit.
    for i in 1..MAX_CAMERAS {
        let camera = VideoCapture::new(i, CAP_ANY);

        let camera = match camera {
            Ok(c) => camera
            Err => break;
        }

        if !camera.is_open() {
            break
        }

        cameras.push(camera);
    }

    println!("There are {} available cameras.", cameras.length());

    // Here we start by opening up all our cameras. This doubles as a method to
    // count the number of available cameras. Not sure how to handle cameras
    // being unplugged while later code is executing, but a problem for another
    // day I guess (maybe I just catch errors and remove broken cameras from the
    // array).

    // TODO: grab frames concurrently (one thread per camera, places 
    // TODO: process frames (maybe draw something on one in leiu of actually storing them)
}
