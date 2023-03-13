"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
function main() {
    const app = (0, express_1.default)();
    app.get("/", function (req, res) {
        if (!req.query || !req.query.city) {
            return res.status(400).send("invalid input");
        }
        ;
        const city = req.query.city.toString().toLowerCase();
        switch (city) {
            case "viborg":
                return res.status(200).send("horrible weather");
            case "skive":
                return res.status(200).send("probably also horrible weather");
            case "copenhagen":
                return res.status(200).send("horrible weather, except they deserve it");
            default:
                return res.status(404).send("unknown city");
        }
    });
    console.log("listening on port 8080");
    app.listen(8080);
}
;
main();
//# sourceMappingURL=main.js.map