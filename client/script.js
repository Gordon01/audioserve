$(function() {
    const baseUrl =`${window.location.protocol}//${window.location.hostname}:3000`;
    function loadFolder(path) {
        $.ajax({
            url: baseUrl+"/folder/"+ path,
            xhrFields: {
                withCredentials: true
             }
            }
            )
        .fail( err => { 
            console.log("Server error", err);
            if (err.status == 404 && path.length) {
                loadFolder("");
            } else if (err.status == 401) {
                $("#login-dialog").modal();
            }
        })
        .then(data => {
            //console.log(data);
            let subfolders = $('#subfolders');
            let count = $('#subfolders-count');
            subfolders.empty();
            count.text(data.subfolders.length);
            for (let subfolder of  data.subfolders) {
                //console.log(subfolder);
                let item = $('<a class="list-group-item list-group-item-action">')
                    .attr("href", subfolder.path)
                    .text(subfolder.name)
                subfolders.append(item);
            }
            if (data.subfolders.length) {
                $("#subfolders-container").show();
            } else {
                $("#subfolders-container").hide();
            }
            let files = $("#files");
            let fcount = $("#files-count");
            files.empty();
            fcount.text(data.files.length);
            for (let file of  data.files) {
                let item = $('<a class="list-group-item list-group-item-action">')
                    .attr("href", file.path)
                    .text(file.name)
                files.append(item);
            }
            if (data.files.length) {
                $("#files-container").show();
            } else {
                $("#files-container").hide();
            }

            update_breadcrumb(path);
            let prevFolder = window.localStorage.getItem("audioserve_folder");
            window.localStorage.setItem("audioserve_folder", path);
            if (prevFolder !== path) {
                clearPlayer();
                }
            let lastFile = window.localStorage.getItem("audioserve_file");
            if (lastFile) {
                let target=$(`#files a[href="${lastFile}"]`);
                if (target.length) {
                    let time = window.localStorage.getItem("audioserve_time");
                    showInView(target);
                    playFile(target, true, time);
                }
            }
        });
    }

    function update_breadcrumb(path) {
        bc = $("#breadcrumb");
        let segments = path.split("/");
        bc.empty();
        bc.append($('<li class="breadcrumb-item"><a href="">Home</a></li>'));
        for (let i=0;  i< segments.length; i++) {
            let item = $('<li class="breadcrumb-item">');
            if (i == segments.length-1) {
                item.addClass("active");
            }
            let partPath = segments.slice(0,i+1).join('/');
            item.append($(`<a href="${partPath}">${segments[i]}</a></li>`));
            bc.append(item);
        }

    }

    function playFile(target, paused, startTime) {
       
        $("#files a").removeClass("active");
        target.addClass("active");
        let path = target.attr("href");
        window.localStorage.setItem("audioserve_file", path);
        let fullUrl = baseUrl+"/audio/"+path;
        let player = $("#player audio").get()[0];
        player.src= fullUrl;
        if (startTime) {
            player.currentTime = startTime
        }
        if (! paused) {
            let res=player.play();
            if (res.catch) {
                res.catch(e => console.log("Play failed", e))
            }
        }
    }

    function clearPlayer() {
        window.localStorage.removeItem("audioserve_file");
        window.localStorage.removeItem("audioserve_time");
        let player = $("#player audio").get()[0];
        player.pause()
        player.src = "";
        $("#files a").removeClass("active");
    }

    function showInView(nextTarget) {
        try {
            nextTarget.get(0).scrollIntoView({block: "center", 
                inline: "nearest",
                behaviour: "smooth"
            });
            }  catch(e) {
                nextTarget.get(0).scrollIntoView();
            } 
    }

    $("#subfolders").on("click", "a.list-group-item-action", evt => {
        loadFolder($(evt.target).attr("href"));
        evt.preventDefault();
    });

    $("#breadcrumb").on("click", "li.breadcrumb-item a", evt => {
        loadFolder($(evt.target).attr("href"));
        evt.preventDefault();
    });

    $("#files").on("click", "a.list-group-item-action", evt => {
        let target = $(evt.target);
        playFile(target);
        evt.preventDefault();
    });

    $("#player audio").on("ended", evt => {
        let nextTarget = $("#files a.active + a");
        if (nextTarget.length) {
            showInView(nextTarget);
            playFile(nextTarget);
        } else {
            clearPlayer();
            console.log("Playback of folder finished");
        }
    });

    $("#player audio").on("timeupdate", evt => {
        window.localStorage.setItem("audioserve_time", evt.target.currentTime);
    });

    $("#login-btn").on("click", evt => {
        let secret = $("#secret-input").val();
        $.ajax({
            url:baseUrl+"/authenticate",
            type: "POST",
            data: {secret: secret},
            xhrFields: {
                withCredentials: true
             }
			
        })
        .done(data => {
            loadFolder(window.localStorage.getItem("audioserve_folder")|| "");
            $("#login-dialog").modal("hide");
        })
        .fail( err => console.log("Login failed", err))
    });

    loadFolder(window.localStorage.getItem("audioserve_folder")|| "");
})