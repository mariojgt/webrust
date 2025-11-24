# Rate Limiting & Throttling System

## Overview

WebRust's Rate Limiting system provides comprehensive protection against abuse while ensuring fair resource distribution. Inspired by Laravel's throttle middleware, this system is designed to be:

- **Easy to use** - Pre-configured strategies for common scenarios
- **Flexible** - Build custom rate limiting rules
- **Fast** - In-memory tracking with minimal overhead
- **Fair** - Per-IP, per-user, or global limiting options
- **Graceful** - Proper HTTP 429 responses with Retry-After headers

---

## Quick Start

### 1. Using Pre-configured Strategies

```rust
use crate::prelude::*;

// Auth endpoint - 5 attempts per 15 minutes
let limiter = auth_limiter();

// API endpoint - 100 requests per minute
let limiter = api_limiter();

// Global limit - 1000 requests per hour
let limiter = global_limiter();

// Check if request allowed
if limiter.check("192.168.1.1").await {
    // Process request
} else {
    // Return 429 Too Many Requests
}
```

### 2. Building Custom Limits

```rust
use crate::prelude::*;

// Custom: 50 requests per 5 minutes per IP
let limiter = RateLimiterBuilder::new()
    .requests(50)
    .window(300)
    .per_ip()
    .exclude_path("/health")
    .build();

if !limiter.check(&ip).await {
    return Err("Rate limited");
}
```

### 3. In Route Handlers

```rust
use axum::extract::ConnectInfo;
use std::net::SocketAddr;

#[post("/api/login")]
async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let limiter = auth_limiter();
    let ip = addr.ip().to_string();
    
    if !limiter.check(&ip).await {
        return Err(Error::too_many_requests(
            limiter.get_remaining(&ip).await,
            limiter.window_seconds()
        ));
    }
    
    // Process login...
    Ok(Json(response))
}
```

---

## Core Concepts

### Rate Limiting Strategies

#### 1. **Auth Limiter** (Very Strict)
```rust
let limiter = auth_limiter();
// 5 attempts per 15 minutes per IP
```

Use for:
- Login attempts
- Password reset
- OTP verification
- 2FA challenges

#### 2. **API Limiter** (Moderate)
```rust
let limiter = api_limiter();
// 100 requests per minute per IP
```

Use for:
- General API endpoints
- Public endpoints
- User actions (create, update)

#### 3. **Global Limiter** (Lenient)
```rust
let limiter = global_limiter();
// 1000 requests per hour per IP
```

Use for:
- Catch-all protection
- Secondary fallback
- Bulk operations

#### 4. **Sensitive Limiter** (Extremely Strict)
```rust
let limiter = sensitive_limiter();
// 10 requests per hour per IP
```

Use for:
- Account deletion
- Permission changes
- API key generation
- Billing operations

#### 5. **Search Limiter** (Moderate)
```rust
let limiter = search_limiter();
// 30 searches per minute per IP
```

Use for:
- Search endpoints
- Filter operations
- Report generation

#### 6. **Upload Limiter** (Strict)
```rust
let limiter = upload_limiter();
// 20 uploads per hour per IP
```

Use for:
- File uploads
- Image processing
- Document submissions

---

## Advanced Usage

### Custom Rate Limiter Builder

```rust
use crate::prelude::*;

// Build from scratch
let limiter = RateLimiterBuilder::new()
    .requests(200)           // Max requests
    .per_minute()            // Time window
    .per_ip()                // Per-IP tracking
    .exclude_path("/health")
    .exclude_path("/status")
    .build();
```

### Builder Methods

```rust
RateLimiterBuilder::new()
    // Set max requests
    .requests(100)
    
    // Time windows
    .per_minute()              // 60 seconds
    .per_hour()                // 3600 seconds
    .per_day()                 // 86400 seconds
    .window(300)               // Custom: 300 seconds
    
    // Tracking method
    .per_ip()                  // Track by IP address
    .per_user()                // Track by user ID
    
    // Exclude paths
    .exclude_path("/health")
    .exclude_paths(vec![...])
    
    // Build
    .build()
```

### Checking Limits

```rust
let limiter = api_limiter();
let key = "192.168.1.1";

// Check if allowed
if limiter.check(&key).await {
    // Request allowed
}

// Get current count
let count = limiter.get_count(&key).await;
println!("Requests: {}", count);

// Get remaining
let remaining = limiter.get_remaining(&key).await;
println!("Remaining: {}", remaining);

// Reset for a key
limiter.reset(&key).await;

// Clear all
limiter.clear().await;
```

---

## Integration with Routes

### Example: Login Route with Rate Limiting

```rust
use axum::extract::ConnectInfo;
use std::net::SocketAddr;
use crate::prelude::*;

#[post("/api/auth/login")]
async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let limiter = auth_limiter();
    let ip = addr.ip().to_string();
    
    // Check rate limit
    if !limiter.check(&ip).await {
        let remaining = limiter.get_remaining(&ip).await;
        let retry_after = limiter.window_seconds();
        
        return Err(Error::response(
            429,
            format!(
                "Too many login attempts. Retry after {} seconds.",
                retry_after
            ),
            Some(remaining),
        ));
    }
    
    // Process authentication
    let user = authenticate(&payload.email, &payload.password).await?;
    
    Ok(Json(AuthResponse {
        token: user.generate_token(),
        user,
    }))
}
```

### Example: Search with Rate Limiting

```rust
#[get("/api/search")]
async fn search(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<SearchParams>,
) -> Result<Json<SearchResults>> {
    let limiter = search_limiter();
    let ip = addr.ip().to_string();
    
    // Limit searches
    if !limiter.check(&ip).await {
        return Err(Error::too_many_requests(
            limiter.get_remaining(&ip).await,
            limiter.window_seconds(),
        ));
    }
    
    // Perform search
    let results = search_db(&params.query).await?;
    Ok(Json(results))
}
```

### Example: File Upload with Rate Limiting

```rust
#[post("/api/files/upload")]
async fn upload_file(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    multipart: Multipart,
) -> Result<Json<FileResponse>> {
    let limiter = upload_limiter();
    let ip = addr.ip().to_string();
    
    // Rate limit uploads
    if !limiter.check(&ip).await {
        return Err(Error::too_many_requests(
            limiter.get_remaining(&ip).await,
            limiter.window_seconds(),
        ));
    }
    
    // Process upload
    let file = process_upload(multipart).await?;
    Ok(Json(FileResponse { file }))
}
```

---

## Response Handling

### HTTP 429 Response

When a request is rate limited, WebRust returns:

```
HTTP/1.1 429 Too Many Requests
Content-Type: application/json
Retry-After: 60

{
  "error": "Too Many Requests",
  "remaining": 0,
  "retry_after": 60
}
```

**Headers:**
- `Retry-After` - Seconds to wait before retrying
- `X-RateLimit-Limit` - Maximum requests
- `X-RateLimit-Remaining` - Requests remaining
- `X-RateLimit-Reset` - Unix timestamp of reset

### Custom Error Response

```rust
if !limiter.check(&ip).await {
    let response = RateLimitResponse {
        limited: true,
        remaining: limiter.get_remaining(&ip).await,
        retry_after: limiter.window_seconds(),
    };
    
    return Ok(response.to_response());
}
```

---

## Best Practices

### 1. Use Appropriate Strategies

```rust
// ✅ GOOD: Use pre-configured strategy
let limiter = auth_limiter();

// ❌ BAD: Create same limiter in multiple places
let limiter = RateLimitConfig::new(5, 900);
```

### 2. Extract IP Correctly

```rust
use axum::extract::ConnectInfo;

// ✅ GOOD: Use ConnectInfo for actual IP
#[post("/login")]
async fn login(ConnectInfo(addr): ConnectInfo<SocketAddr>) {
    let ip = addr.ip().to_string();
}

// ⚠️ BEHIND PROXY: Use X-Forwarded-For header
fn get_client_ip(headers: &HeaderMap, addr: ConnectInfo<SocketAddr>) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .unwrap_or_else(|| &addr.0.ip().to_string())
        .to_string()
}
```

### 3. Exclude Health Endpoints

```rust
// ✅ GOOD: Exclude monitoring endpoints
RateLimiterBuilder::new()
    .exclude_path("/health")
    .exclude_path("/metrics")
    .exclude_path("/status")
    .build()

// ❌ BAD: Allow unlimited monitoring requests to add load
```

### 4. Chain Limiters for Layered Protection

```rust
// Layer 1: Per-operation limit (strict)
let operation_limiter = auth_limiter(); // 5 per 15 min

// Layer 2: Global limit (lenient)
let global_limiter = global_limiter();  // 1000 per hour

// Check both
if !operation_limiter.check(&ip).await {
    return Err("Operation rate limited");
}

if !global_limiter.check(&ip).await {
    return Err("Global rate limited");
}
```

### 5. Distinguish Between Abuse and Legitimate Spike

```rust
// ✅ GOOD: Higher limits for legitimate spikes
RateLimiterBuilder::new()
    .requests(100)
    .per_minute()
    .build()

// ❌ BAD: Too strict, blocks legitimate users
RateLimiterBuilder::new()
    .requests(5)
    .per_minute()
    .build()
```

---

## Common Patterns

### Pattern 1: Progressive Delays

```rust
let limiter = api_limiter();

for attempt in 1..=3 {
    if limiter.check(&ip).await {
        // Success
        return Ok(response);
    }
    
    // Exponential backoff
    tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt))).await;
}

Err("Rate limited")
```

### Pattern 2: User-Specific Limits

```rust
let user_id = user.id;
let limiter = RateLimiterBuilder::new()
    .requests(500)
    .per_hour()
    .build();

if !limiter.check(&user_id.to_string()).await {
    return Err("User rate limited");
}
```

### Pattern 3: Tiered Limits Based on Role

```rust
let limit = match user.role {
    Role::Admin => 10000,      // 10k per hour
    Role::Premium => 5000,     // 5k per hour
    Role::Basic => 1000,       // 1k per hour
};

let limiter = RateLimiterBuilder::new()
    .requests(limit)
    .per_hour()
    .build();
```

### Pattern 4: Bypass for Trusted IPs

```rust
let trusted_ips = vec!["127.0.0.1", "192.168.1.0"];

if !trusted_ips.contains(&ip.as_str()) {
    if !limiter.check(&ip).await {
        return Err("Rate limited");
    }
}
```

---

## Configuration Reference

### RateLimitConfig

```rust
pub struct RateLimitConfig {
    pub max_requests: u32,              // Max requests allowed
    pub window_seconds: u64,            // Time window
    pub per_ip: bool,                   // Track per IP
    pub per_user: bool,                 // Track per user
    pub exclude_paths: Vec<String>,     // Exempt paths
}
```

### Preset Strategies

| Strategy | Requests | Window | Use Case |
|----------|----------|--------|----------|
| `auth_limiter()` | 5 | 15 min | Login, password reset |
| `api_limiter()` | 100 | 1 min | General API endpoints |
| `global_limiter()` | 1000 | 1 hour | Fallback protection |
| `sensitive_limiter()` | 10 | 1 hour | Account deletion, billing |
| `search_limiter()` | 30 | 1 min | Search queries |
| `upload_limiter()` | 20 | 1 hour | File uploads |

---

## Monitoring & Diagnostics

### Check Rate Limit Status

```rust
let limiter = api_limiter();
let ip = "192.168.1.1";

// Current usage
let count = limiter.get_count(&ip).await;
let remaining = limiter.get_remaining(&ip).await;

println!("Used: {}/{}", count, limiter.max_requests());
println!("Remaining: {}", remaining);
```

### Reset User's Limit

```rust
// After successful password reset
limiter.reset(&ip).await;

// User can now retry login
```

### Clear All Limits

```rust
// During maintenance or reset
limiter.clear().await;
```

---

## Testing Rate Limits

### Unit Tests

```rust
#[tokio::test]
async fn test_rate_limit() {
    let limiter = RateLimiter::new(RateLimitConfig::new(3, 60));
    
    assert!(limiter.check("user1").await);
    assert!(limiter.check("user1").await);
    assert!(limiter.check("user1").await);
    assert!(!limiter.check("user1").await); // 4th request blocked
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_auth_endpoint_rate_limit() {
    let client = test_client().await;
    
    // Make 5 successful requests
    for _ in 0..5 {
        assert_eq!(
            client.post("/api/login")
                .json(&login_payload())
                .await
                .status(),
            429
        );
    }
    
    // 6th request should be rate limited
    let response = client.post("/api/login")
        .json(&login_payload())
        .await;
    assert_eq!(response.status(), 429);
}
```

---

## Troubleshooting

### Issue: All requests are rate limited

**Cause:** Key is being blocked immediately

**Solution:**
```rust
// Check if rate limiter is reset between tests
limiter.clear().await;

// Or use different keys
limiter.check("user1").await;  // Different keys
limiter.check("user2").await;
```

### Issue: Legitimate users getting rate limited

**Cause:** Limits are too strict

**Solution:**
```rust
// Increase limits
RateLimiterBuilder::new()
    .requests(500)      // Increased from 100
    .per_hour()         // Changed from per_minute
    .build()
```

### Issue: Rate limiter not blocking abuse

**Cause:** Attacker using different IPs

**Solution:**
```rust
// Add captcha or stronger validation
// Or use per-user limits if authenticated
if let Some(user) = extract_user(&req).await {
    limiter.check(&user.id.to_string()).await
} else {
    limiter.check(&ip).await
}
```

---

## Comparison with Laravel

| Feature | Laravel Throttle | WebRust |
|---------|------------------|---------|
| IP-based limiting | ✅ | ✅ |
| User-based limiting | ✅ | ✅ |
| Custom windows | ✅ | ✅ |
| Middleware integration | ✅ | ✅ |
| **Async-first** | ❌ | ✅ |
| **Type-safe** | ⚠️ | ✅ |
| **Strategies** | Basic | Advanced |
| **Performance** | Good | Excellent |

---

## Next Steps

1. **Implement in your routes** - Add rate limiting to critical endpoints
2. **Choose strategies** - Select appropriate limits for each endpoint
3. **Test thoroughly** - Verify limits work as expected
4. **Monitor** - Track rate limit violations
5. **Adjust** - Fine-tune limits based on metrics

## Quick Links

- **Quick Reference:** `RATE_LIMITING_QUICK_REF.md`
- **API Docs:** `src/http/rate_limiter.rs`
- **Strategies:** `src/http/rate_limit_strategies.rs`
