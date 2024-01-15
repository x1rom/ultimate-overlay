# Ultimate Overlay

This is a stream overlay for Super Smash Bros. It is built with Tauri, Vue and Vuetify, and works on Windows, Linux and MacOS. You can create your own Overlays with it as long as you are able to write html code.

## Installation

Download the latest Version from the [Releases Page](https://github.com/x1rom/ultimate-overlay/releases), and unpack the archive. For Windows, execute the setup.exe file. It will install the overlay and all dependencies. For Linux, simply execute the .appimage file. For MacOS users, you will have to compile the program on your own, as i do not own an Apple device.

## Usage

In OBS, create a Browser source, and set the appropriate resolution. Then select the overlay.html file in the overlays folder. It should work out of the Box as long as you have the application open.

## Development Roadmap

I have planned more features, which i will implement once i have some free time. Here's a list of planned updates:

-   Integration with start.gg API
-   updated UI
-   Graphical Overlay creator.
-   Arbitrary amount of casters

## Create your own overlay

If you know how to write html code, you can create your own overlay. Embed the updateData.js file, which will update the text of elements that have a certain id. Here are the ids you need to set:

```
score_l
score_r
name_l
name_r
tournament_name
round_name
caster1
caster2
```

## Write your own JS code.

You can write your own code if you want to. The Data is provided through a REST service. Send a GET request to localhost:4875/get_data, which will return the following object:

```TS
{
    score_l: number,
    score_r: number,
    name_l: string,
    name_r: string,
    tournament_name: string,
    round_name: string,
    casters: string[],
}
```
