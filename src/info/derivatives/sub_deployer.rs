use core::mem::replace;

use alloy_primitives::Address;
use derive_more::{Display, IsVariant};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use triomphe::Arc;

/// The HIP-3* sub-deployer grant operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IsVariant, Display)]
#[display("{}", self.as_str())]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum SubDeployerGrantOperation {
  /// Grants the ability to modify approvals for the sub-deployer.
  ModifyApproval,

  /// Grants the ability to modify backstop liquidator approvals for the sub-deployer.
  ModifyBackstopLiquidatorApproval,

  /// Grants the ability to set the reduce-only flag for the sub-deployer.
  SetReduceOnly,

  /// Grants the ability to cancel orders for the sub-deployer.
  Cancel,

  /// Grants the ability to cancel all orders for the sub-deployer.
  CancelAll,
  /// Grants the ability to place orders for the sub-deployer.
  Order,
  /// Grants the ability to send assets for the sub-deployer.
  SendAsset,
  /// Grants the ability to set the oracle for the sub-deployer.
  SetOracle,
}

impl SubDeployerGrantOperation {
  /// Returns a human-readable name for the grant.
  #[inline(always)]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::ModifyApproval => "modifyApproval",
      Self::ModifyBackstopLiquidatorApproval => "modifyBackstopLiquidatorApproval",
      Self::SetReduceOnly => "setReduceOnly",
      Self::Cancel => "cancel",
      Self::CancelAll => "cancelAll",
      Self::Order => "order",
      Self::SendAsset => "sendAsset",
      Self::SetOracle => "setOracle",
    }
  }
}

/// Represents a grant for a sub-deployer, specifying the HIP-3* operation it allows.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct SubDeployerGrant {
  hip3_star: SubDeployerGrantOperation,
}

impl SubDeployerGrant {
  /// Returns the HIP-3* operation associated with the grant.
  #[inline(always)]
  pub const fn hip3_star(&self) -> SubDeployerGrantOperation {
    self.hip3_star
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
enum Source {
  FunctionName(SmolStr),
  Grant(SubDeployerGrant),
}

/// Delegated function names and their authorized executor addresses
#[derive(Debug, Clone)]
pub struct SubDeployer {
  source: Source,
  executors: Arc<[Address]>,
}

impl SubDeployer {
  /// Returns a reference to the function name if it exists.
  #[inline(always)]
  pub const fn function_name(&self) -> Option<&SmolStr> {
    match &self.source {
      Source::FunctionName(name) => Some(name),
      Source::Grant(_) => None,
    }
  }

  /// Returns a reference to the grant if it exists.
  #[inline(always)]
  pub const fn grant(&self) -> Option<&SubDeployerGrant> {
    match &self.source {
      Source::FunctionName(_) => None,
      Source::Grant(grant) => Some(grant),
    }
  }

  /// Returns a reference to the executors.
  #[inline(always)]
  pub fn executors(&self) -> &[Address] {
    self.executors.as_ref()
  }

  /// Takes the executors out of the `SubDeployer`, leaving an empty `Vec<Address>`.
  #[inline(always)]
  pub fn take_executors(&mut self) -> Arc<[Address]> {
    replace(&mut self.executors, Arc::from_iter([]))
  }
}

#[cfg(feature = "serde")]
const _: () = {
  impl Serialize for SubDeployer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      let executors: &[_] = self.executors();
      (&self.source, executors).serialize(serializer)
    }
  }

  impl<'de> Deserialize<'de> for SubDeployer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'de>,
    {
      <(Source, Vec<Address>)>::deserialize(deserializer).map(|(source, executors)| Self {
        source,
        executors: executors.into(),
      })
    }
  }
};
