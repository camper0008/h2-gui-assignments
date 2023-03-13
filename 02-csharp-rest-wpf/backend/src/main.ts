import express, { Request, Response } from "express";

function main() {
    const app = express();
    app.get("/", function(req: Request, res: Response) {
        if (!req.query || !req.query.city) { return res.status(400).send("invalid input") };

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
};

main();
