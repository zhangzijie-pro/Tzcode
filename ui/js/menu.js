const {appWindow} = window.__TAURI__.window;
import "./setting"
import { leftfileValue } from "./setting";

// Minimize window
document.getElementById('titlebar-minimize').addEventListener('click', () => {
    appWindow.minimize()
});

// Maximize window
document.getElementById('titlebar-maximize').addEventListener('click', () => {
    appWindow.toggleMaximize()
});

// Close window
document.getElementById('titlebar-close').addEventListener('click', () => {
    appWindow.close()
});


// toggle color

function changeBackgroundColor(selectors, newColor,originalColor,isOriginalColor) {
    selectors.forEach(selector => {
        var elements = document.querySelectorAll(selector);
        elements.forEach(element => {
            if (isOriginalColor) {
                element.style.backgroundColor = newColor;
            } else {
                element.style.backgroundColor = originalColor;
            }
        });
    });
}

function change_color(rfv,lfv,ce,tl,menu,ft,isOriginalColor){
    changeBackgroundColor(['.titlebar'],menu,'#329ea3',isOriginalColor);
    changeBackgroundColor(['.footer'],ft,'#1e1e1e',isOriginalColor);
    changeBackgroundColor(['.bottom-section','input'],tl,'#252526',isOriginalColor);
    changeBackgroundColor(['.CodeMirror'],ce,'#e4e3e3 !important',isOriginalColor);
    changeBackgroundColor(['.initial-page','.top-section'],ce,'#1e1e1e',isOriginalColor);
    changeBackgroundColor(['.left-file'],lfv,'#3c3c3c',isOriginalColor);
    changeBackgroundColor(['.right-file','.sidebar'],rfv,'#2c2c2c',isOriginalColor);
}

let isOriginalColor = true;
document.getElementById('titlebar-background').addEventListener('click',()=>{
/*     changeBackgroundColor(['.titlebar'],'#8f9092','#329ea3',isOriginalColor);
    changeBackgroundColor(['.footer'],'#8f9092','#1e1e1e',isOriginalColor);
    changeBackgroundColor(['.bottom-section','input'],'','#252526',isOriginalColor);
    changeBackgroundColor(['.CodeMirror'],'','#e4e3e3 !important',isOriginalColor);
    changeBackgroundColor(['.initial-page','.top-section'],'','#1e1e1e',isOriginalColor);
    changeBackgroundColor(['.left-file'],'#fff','#3c3c3c',isOriginalColor);
    changeBackgroundColor(['.right-file','.sidebar'],'#ff7','#2c2c2c',isOriginalColor);
    
    isOriginalColor=!isOriginalColor; */
    change_color(rightfileValue,leftfileValue,codeEditorValue,terimalValue,menuValue,footerValue,isOriginalColor);
    isOriginalColor=!isOriginalColor;
})