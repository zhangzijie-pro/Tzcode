// 扩展
async function new_window(){
    await invoke('open_new_window');
}

document.getElementById('home-file').addEventListener('click', function() {
    document.getElementById('home-file').style.borderLeft="3px solid #FFFFFF"
    document.getElementById('search-file').style.borderLeft=""
    document.getElementById('user-group').style.borderLeft=""

    document.getElementById('fileList').style.display = 'block';
    document.getElementById('searchContainer').style.display = 'none';
    document.getElementById('user_send').style.display = 'none';
});

document.getElementById('search-file').addEventListener('click', function() {
    document.getElementById('search-file').style.borderLeft="3px solid #FFFFFF"
    document.getElementById('home-file').style.borderLeft=""
    document.getElementById('user-group').style.borderLeft=""

    document.getElementById('fileList').style.display = 'none';
    document.getElementById('searchContainer').style.display = 'flex';
    document.getElementById('user_send').style.display = 'none';

});

document.getElementById('user-group').addEventListener('click', function() {
    document.getElementById('home-file').style.borderLeft=""
    document.getElementById('search-file').style.borderLeft=""
    document.getElementById('user-group').style.borderLeft="3px solid #FFFFFF"

    document.getElementById('fileList').style.display = 'none';
    document.getElementById('searchContainer').style.display = 'none';
    document.getElementById('user-group').style.display='flex';
    document.getElementById('user_send').style.display = 'flex';
});

document.getElementById('setting').addEventListener('click', async function(){
    await invoke('open_setting');
})