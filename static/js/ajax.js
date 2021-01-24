function join(myId, aid, gid, uid) {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 202) {
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
            document.getElementById(myId).setAttribute("class", "list-group-item list-group-item-light");
            document.getElementById(myId).setAttribute("title", "anmelden");
            document.getElementById(myId).setAttribute("onclick", `join(${myId}, ${aid}, ${gid}, ${uid})` );
        }
    };
    xhttp.open("POST", "/participate/revoke", true);
    xhttp.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    xhttp.send("aid=" + aid + "&gid=" + gid + "&uid=" + uid);
}

function getAppointmentInfo(aid) {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function () {
        if (this.readyState === 4 && this.status === 200) {
            var info = JSON.parse(this.responseText);
            showAppointmentInfo(info);
        }
    }
    xhttp.open("GET", `participate/${aid}/info`, true);
    xhttp.send();
}

function showAppointmentInfo(json_data) {
    document.getElementById("appointmentModalTitle").innerHTML = json_data.appointment.title;
    document.getElementById("appointmentModalKeyInfo").innerHTML =
        `<strong>von:</strong> ${json_data.appointment.begins}<br><strong>bis:</strong> ${json_data.appointment.ends}<br><strong>Ort:</strong> ${json_data.appointment.place}`;
    document.getElementById("appointmentModalDescription").innerHTML = json_data.appointment.description;

    var attendees = "";
    var idx = 1;
    for(i = 0; i < json_data.groups.length; i++) {
        for(j = 0; j < json_data.groups[i][1].length; j++) {
            attendees += `<tr><th scope='row'>${idx}</th>`;
            attendees += `<td>${json_data.groups[i][1][j].first_name}</td>`;
            attendees += `<td>${json_data.groups[i][1][j].last_name}</td>`;
            attendees += `<td>${json_data.groups[i][1][j].email}</td>`;
            attendees += `<td>${json_data.groups[i][0].title}</td>`;
            idx += 1;
        }
    }
    document.getElementById("appointmentModalSignedUp").innerHTML = attendees;

    $("#appointmentModal").modal("show");
    console.log(json_data);
}