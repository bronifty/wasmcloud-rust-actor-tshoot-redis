import { createClient } from 'redis';

// Create a Redis client
const client = createClient({
    socket: {
        host: 'localhost',
        port: 6379
    }
});
// const client = createClient({
//     password: 'asdf',
//     socket: {
//         host: 'redis-11374.c326.us-east-1-3.ec2.cloud.redislabs.com',
//         port: 11374
//     }
// });

// Function to add a name to the set
async function addNameToSet(name) {
    try {
        // Connect to the Redis server
        await client.connect();

        // Add the name to the set 'names'
        const result = await client.sAdd('names', name);

        // Output the result
        console.log(`Name added: ${result}`);

        // Disconnect the client
        await client.disconnect();
    } catch (error) {
        console.error('Error:', error);
    }
}

// Add 'Bob' to the set
addNameToSet('ffej');
