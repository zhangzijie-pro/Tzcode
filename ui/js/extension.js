// 扩展
async function new_window(){
    await invoke('open_new_window');
}

document.getElementById('setting').addEventListener('click', async function(){
    await invoke('open_setting');
})