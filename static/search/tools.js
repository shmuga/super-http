document.addEventListener('click', function(e){
    if(e.target && e.target.classList.contains('tag')) {
      window.location = `/search?q=tag:${e.target.innerHTML}`;
    }
});

