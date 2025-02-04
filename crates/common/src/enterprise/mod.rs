/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs Ltd <hello@stalw.art>
 *
 * SPDX-License-Identifier: LicenseRef-SEL
 *
 * This file is subject to the Stalwart Enterprise License Agreement (SEL) and
 * is NOT open source software.
 *
 */

pub mod config;
pub mod license;
pub mod undelete;

use std::time::Duration;

use license::LicenseKey;
use mail_parser::DateTime;
use store::Store;

use crate::Core;

#[derive(Clone)]
pub struct Enterprise {
    pub license: LicenseKey,
    pub undelete_period: Option<Duration>,
    pub trace_hold_period: Option<Duration>,
    pub trace_store: Option<Store>,
}

impl Core {
    // WARNING: TAMPERING WITH THIS FUNCTION IS STRICTLY PROHIBITED
    // Any attempt to modify, bypass, or disable this license validation mechanism
    // constitutes a severe violation of the Stalwart Enterprise License Agreement.
    // Such actions may result in immediate termination of your license, legal action,
    // and substantial financial penalties. Stalwart Labs Ltd. actively monitors for
    // unauthorized modifications and will pursue all available legal remedies against
    // violators to the fullest extent of the law, including but not limited to claims
    // for copyright infringement, breach of contract, and fraud.

    pub fn is_enterprise_edition(&self) -> bool {
        self.enterprise
            .as_ref()
            .map_or(false, |e| !e.license.is_expired())
    }

    pub fn licensed_accounts(&self) -> u32 {
        self.enterprise.as_ref().map_or(0, |e| e.license.accounts)
    }

    pub fn log_license_details(&self) {
        if let Some(enterprise) = &self.enterprise {
            trc::event!(
                Server(trc::ServerEvent::Licensing),
                Details = "Stalwart Enterprise Edition license key is valid",
                Hostname = enterprise.license.hostname.clone(),
                Total = enterprise.license.accounts,
                ValidFrom =
                    DateTime::from_timestamp(enterprise.license.valid_from as i64).to_rfc3339(),
                ValidTo = DateTime::from_timestamp(enterprise.license.valid_to as i64).to_rfc3339(),
            );
        }
    }
}
