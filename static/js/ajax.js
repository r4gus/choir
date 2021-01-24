function join(myId, aid, gid, uid) {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 202) {
            console.log("join: " + myId);
            document.getElementById(myId).setAttribute("class", "list-group-item list-group-item-success");
            document.getElementById(myId).setAttribute("title", "abmelden");
            document.getElementById(myId).setAttribute("onclick", `revoke(${myId}, ${aid}, ${gid}, ${uid})` );
        }
    };
    xhttp.open("POST", "/participate/join", true);
    xhttp.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    xhttp.send("aid=" + aid + "&gid=" + gid + "&uid=" + uid);
}

function revoke(myId, aid, gid, uid) {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 202) {
            console.log("revoke: " + myId);
            document.getElementById(myId).setAttribute("class", "list-group-item list-group-item-light");
            document.getElementById(myId).setAttribute("title", "anmelden");
            document.getElementById(myId).setAttribute("onclick", `join(${myId}, ${aid}, ${gid}, ${uid})` );
        }
    };
    xhttp.open("POST", "/participate/revoke", true);
    xhttp.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    xhttp.send("aid=" + aid + "&gid=" + gid + "&uid=" + uid);
}