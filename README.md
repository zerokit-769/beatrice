
a serverless v2ray tunnel

## Deploy

1. [Create an API token](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/) from the cloudflare dashboard.
2. Set GitHub Repository Secret

Navigate to: GitHub → Your Repo → Settings → Secrets and variables → Actions
Add a new secret:
Name: CLOUDFLARE_API_TOKEN
Value: Your API token
| Variable            | Description                                      |
|---------------------|--------------------------------------------------|
| CLOUDFLARE_API_TOKEN | The API key retrieved from Cloudflare dashboard |

3.Open the Actions tab on GitHub,Enable workflows if prompted.

