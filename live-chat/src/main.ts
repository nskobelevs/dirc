import app from "./server.ts"

const port = +(Deno.env.get("PORT") || "8080");

console.log(`Listening on port ${port}...`);

await app.listen({ port });