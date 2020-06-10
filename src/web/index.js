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

class VxApp extends HTMLElement {
  static get observedAttributes() {
    return ["loading", "flags"];
  }

  connectedCallback() {
    this.fetchFlags();
  }

  attributeChangedCallback() {
    this.render();
  }

  async fetchFlags() {
    this.loading = true;
    try {
      this.flags = await request("/api/flags");
    } finally {
      this.loading = false;
    }
  }

  openModal() {
    $(".ui.modal form input").val("");
    $(".ui.modal").modal("show");
  }

  async closeModal() {
    const name = $("form.ui.form")[0].name.value;
    await request("/api/flags", "POST", { name });
    $(".ui.modal").modal("hide");
    await this.fetchFlags();
  }

  render() {
    let content = `<vx-content flags='${JSON.stringify(this.flags)}'></vx-content>`;

    if (this.loading) {
      content = "<vx-loader></vx-loader>";
    }

    if (this.flags && this.flags.length === 0) {
      content = "<vx-empty-content></vx-empty-content>";
    }

    this.innerHTML = `
      <div class="ui text container">
        <vx-header></vx-header>
        ${content}
        <vx-modal></vx-modal>
      </div>
    `;

    $("#add-flag-btn.ui.primary.button").click(() => {
      this.openModal();
    });

    $("#save-flag-btn.ui.primary.button").click(() => {
      this.closeModal();
    });
  }

  get loading() {
    return JSON.parse(this.getAttribute("loading"));
  }

  set loading(v) {
    this.setAttribute("loading", JSON.stringify(v));
  }

  get flags() {
    return JSON.parse(this.getAttribute("flags"));
  }

  set flags(v) {
    this.setAttribute("flags", JSON.stringify(v));
  }
}

class VxHeader extends HTMLElement {
  connectedCallback() {
    this.innerHTML = document.getElementById("vx-header").innerHTML;
  }
}

class VxLoader extends HTMLElement {
  connectedCallback() {
    this.innerHTML = document.getElementById("vx-loader").innerHTML;
  }
}

class VxEmptyContent extends HTMLElement {
  connectedCallback() {
    this.innerHTML = document.getElementById("vx-empty-content").innerHTML;
  }
}

class VxContent extends HTMLElement {
  static get observedAttributes() {
    return ["flags"];
  }

  attributeChangedCallback() {
    this.render();
  }

  render() {
    this.innerHTML = document.getElementById("vx-content").innerHTML;

    const rows = this.flags.reduce((acc, flag) => {
      return (acc += `
        <tr>
          <td>${flag.name}</td>
          <td>
            <div class="ui toggle checkbox" data-name="${flag.name}">
              <input type="checkbox" tabindex="0" class="hidden" ${flag.is_enabled ? "checked" : ""}>
              <label>${flag.is_enabled ? "On" : "Off"}</label>
            </div>
          </td>
        </tr>
      `);
    }, "");

    this.querySelector("tbody").innerHTML = rows;

    $(".ui.checkbox")
      .checkbox()
      .change(async function () {
        const name = $(this).data("name");
        const is_enabled = $(this).hasClass("checked");
        await request("/api/flags", "PUT", { name, is_enabled });
        $(this)
          .find("label")
          .html(is_enabled ? "On" : "Off");
      });
  }

  get flags() {
    return JSON.parse(this.getAttribute("flags"));
  }
}

class VxModal extends HTMLElement {
  connectedCallback() {
    this.innerHTML = document.getElementById("vx-modal").innerHTML;
  }
}

window.customElements.define("vx-app", VxApp);
window.customElements.define("vx-header", VxHeader);
window.customElements.define("vx-loader", VxLoader);
window.customElements.define("vx-empty-content", VxEmptyContent);
window.customElements.define("vx-content", VxContent);
window.customElements.define("vx-modal", VxModal);
