# Authentication & Identity Flow

SharpSystem's authentication is designed as a secure, multi-step "Bootstrap" protocol that initializes both user identity and organizational context.

## ◈ Multi-Step Initialization

The registration wizard follows a strict validation sequence to ensure data integrity:

1. **Step 0: Bootstrap**: Introductory layer to explain the initialization process.
2. **Step 1: Organization**: Creation of the parent tenant.
3. **Step 2: Identity**: Username selection (must be unique).
4. **Step 3: Protocol**: Email verification.
5. **Step 4: Security**: Private key (password) generation with "Peep" visibility toggle.
6. **Step 5: Review**: Final verification before server-side commit.

## ◈ Technical Implementation

- **Server-Side**: Registration logic is handled in `actions/src/auth.rs` via the `Signup` server function.
- **Client-Side**: Managed in `src/routes/auth/register.rs` using Leptos signals and granular validation.
- **Payloads**: The `SignupPayload` in `models/src/payloads/auth.rs` ensures type safety across the network boundary.

## ◈ Security Measures

- **Validation**: Strict client-side blocking combined with server-side re-validation.
- **Feedback**: Integrated with the Notification Engine to provide immediate clarity on data requirements.
- **UI Architecture**: Uses a glassmorphic terminal aesthetic to reinforce the system's focus on high-fidelity operations.
