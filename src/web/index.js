async function init() {
  bindEvents();
  await fetchFlags();
}

function bindEvents() {
  $("#add-flag-btn.ui.primary.button").click(() => {
    $(".ui.modal form input").val("");
    $(".ui.modal").modal("show");
  });
  $("#save-flag-btn.ui.primary.button").click(async () => {
    const name = $("form.ui.form")[0].name.value;
    await request("/api/flags", "POST", { name });
    $(".ui.modal").modal("hide");
    await fetchFlags();
  });
}

async function fetchFlags() {
  $(".vexil-home-empty-content-container").addClass("vexil-hidden");
  $(".vexil-home-loaded-content-container").addClass("vexil-hidden");
  $(".vexil-home-loading-content-container").removeClass("vexil-hidden");

  const flags = await request("/api/flags");

  $(".vexil-home-loading-content-container").addClass("vexil-hidden");

  if (flags.length === 0) {
    $(".vexil-home-empty-content-container").removeClass("vexil-hidden");
    return;
  }

  const rows = flags.reduce((acc, flag) => {
    const statusText = flag.is_enabled ? "On" : "Off";
    const checked = flag.is_enabled ? "checked" : "";
    return (acc += `
          <tr>
            <td>${flag.name}</td>
            <td>
              <div class="ui toggle checkbox" data-name="${flag.name}">
                <input type="checkbox" tabindex="0" class="hidden" ${checked}>
                <label>${statusText}</label>
              </div>
            </td>
          </tr>
          `);
  }, "");

  $(".vexil-home-loaded-content-container tbody").html(rows);
  $(".ui.checkbox")
    .checkbox()
    .change(async function () {
      const name = $(this).data("name");
      const is_enabled = $(this).hasClass("checked");
      await request("/api/flags", "PUT", { name, is_enabled });
    });
  $(".vexil-home-loaded-content-container").removeClass("vexil-hidden");
}

async function request(url, method = "GET", body) {
  const response = await fetch(url, {
    method,
    body: body ? JSON.stringify(body) : undefined,
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (response.ok) {
    return response.json();
  } else {
    throw new Error(response.statusText);
  }
}
