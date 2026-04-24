import http from 'k6/http';
import { check, sleep } from 'k6';

// Configure the load test: 50 simultaneous users hammering the server for 15 seconds
export const options = {
    vus: 50,
    duration: '15s',
};

const BASE_URL = 'http://127.0.0.1:3000';

export default function () {
    // 1. Test the POST /shorten endpoint
    // We use __VU (Virtual User ID) and __ITER (Iteration number) to make the URL unique every time,
    // forcing the database to do actual inserts rather than hitting constraints.
    const payload = JSON.stringify({
        url: `https://example.com/rust-test/${__VU}/${__ITER}`
    });

    const params = {
        headers: { 'Content-Type': 'application/json' },
    };

    const shortenRes = http.post(`${BASE_URL}/shorten`, payload, params);

    check(shortenRes, {
        'shorten status is 200': (r) => r.status === 200,
    });

    // 2. Test the GET /{code} redirect endpoint
    if (shortenRes.status === 200) {
        const code = shortenRes.json('code');

        // We tell k6 NOT to automatically follow redirects (maxRedirects: 0)
        // because we just want to verify our server returns the 307 Temporary Redirect status.
        const redirectRes = http.get(`${BASE_URL}/${code}`, { redirects: 0 });

        check(redirectRes, {
            'redirect status is 307': (r) => r.status === 307,
        });
    }

    // A tiny pause to simulate real network gaps
    sleep(0.05);
}