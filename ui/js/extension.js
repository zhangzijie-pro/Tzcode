// 扩展
async function new_window(){
    await invoke('open_new_window');
}

document.getElementById('home-file').addEventListener('click', function() {
    document.getElementById('fileList').style.display = 'block';
    document.getElementById('searchContainer').style.display = 'none';
});

document.getElementById('search-file').addEventListener('click', function() {
    document.getElementById('fileList').style.display = 'none';
    document.getElementById('searchContainer').style.display = 'flex';
});

document.getElementById('setting').addEventListener('click', async function(){
    await invoke('open_setting');
})