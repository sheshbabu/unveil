import * as React from "react";
import * as ReactDOM from "react-dom";
import App from "./App";
import "semantic-ui-css/semantic.min.css";
import "./index.css";

// @ts-ignore
if (module.hot) module.hot.accept();

ReactDOM.render(<App />, document.getElementById("root"));
