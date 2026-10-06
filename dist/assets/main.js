document.addEventListener("pointermove", (event) => {
    document.documentElement.style.setProperty(
        "--mouse-x",
        `${event.clientX}px`
    );

    document.documentElement.style.setProperty(
        "--mouse-y",
        `${event.clientY}px`
    );
});