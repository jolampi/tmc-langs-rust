(function() {var implementors = {};
implementors["j4rs"] = [{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"j4rs/enum.InvocationArg.html\" title=\"enum j4rs::InvocationArg\">InvocationArg</a>","synthetic":false,"types":["j4rs::api::InvocationArg"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"j4rs/struct.Instance.html\" title=\"struct j4rs::Instance\">Instance</a>","synthetic":false,"types":["j4rs::api::Instance"]}];
implementors["oauth2"] = [{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.ClientId.html\" title=\"struct oauth2::ClientId\">ClientId</a>","synthetic":false,"types":["oauth2::ClientId"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.AuthUrl.html\" title=\"struct oauth2::AuthUrl\">AuthUrl</a>","synthetic":false,"types":["oauth2::AuthUrl"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.TokenUrl.html\" title=\"struct oauth2::TokenUrl\">TokenUrl</a>","synthetic":false,"types":["oauth2::TokenUrl"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.RedirectUrl.html\" title=\"struct oauth2::RedirectUrl\">RedirectUrl</a>","synthetic":false,"types":["oauth2::RedirectUrl"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.ResponseType.html\" title=\"struct oauth2::ResponseType\">ResponseType</a>","synthetic":false,"types":["oauth2::ResponseType"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.ResourceOwnerUsername.html\" title=\"struct oauth2::ResourceOwnerUsername\">ResourceOwnerUsername</a>","synthetic":false,"types":["oauth2::ResourceOwnerUsername"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.Scope.html\" title=\"struct oauth2::Scope\">Scope</a>","synthetic":false,"types":["oauth2::Scope"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.PkceCodeChallengeS256.html\" title=\"struct oauth2::PkceCodeChallengeS256\">PkceCodeChallengeS256</a>","synthetic":false,"types":["oauth2::PkceCodeChallengeS256"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.PkceCodeChallengeMethod.html\" title=\"struct oauth2::PkceCodeChallengeMethod\">PkceCodeChallengeMethod</a>","synthetic":false,"types":["oauth2::PkceCodeChallengeMethod"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.ClientSecret.html\" title=\"struct oauth2::ClientSecret\">ClientSecret</a>","synthetic":false,"types":["oauth2::ClientSecret"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.CsrfToken.html\" title=\"struct oauth2::CsrfToken\">CsrfToken</a>","synthetic":false,"types":["oauth2::CsrfToken"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.PkceCodeVerifierS256.html\" title=\"struct oauth2::PkceCodeVerifierS256\">PkceCodeVerifierS256</a>","synthetic":false,"types":["oauth2::PkceCodeVerifierS256"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.AuthorizationCode.html\" title=\"struct oauth2::AuthorizationCode\">AuthorizationCode</a>","synthetic":false,"types":["oauth2::AuthorizationCode"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.RefreshToken.html\" title=\"struct oauth2::RefreshToken\">RefreshToken</a>","synthetic":false,"types":["oauth2::RefreshToken"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.AccessToken.html\" title=\"struct oauth2::AccessToken\">AccessToken</a>","synthetic":false,"types":["oauth2::AccessToken"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.EmptyExtraTokenFields.html\" title=\"struct oauth2::EmptyExtraTokenFields\">EmptyExtraTokenFields</a>","synthetic":false,"types":["oauth2::EmptyExtraTokenFields"]},{"text":"impl&lt;EF:&nbsp;<a class=\"trait\" href=\"oauth2/trait.ExtraTokenFields.html\" title=\"trait oauth2::ExtraTokenFields\">ExtraTokenFields</a>, TT:&nbsp;<a class=\"trait\" href=\"oauth2/trait.TokenType.html\" title=\"trait oauth2::TokenType\">TokenType</a>&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.StandardTokenResponse.html\" title=\"struct oauth2::StandardTokenResponse\">StandardTokenResponse</a>&lt;EF, TT&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TT: <a class=\"trait\" href=\"oauth2/trait.TokenType.html\" title=\"trait oauth2::TokenType\">TokenType</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;EF: <a class=\"trait\" href=\"oauth2/trait.ExtraTokenFields.html\" title=\"trait oauth2::ExtraTokenFields\">ExtraTokenFields</a>,&nbsp;</span>","synthetic":false,"types":["oauth2::StandardTokenResponse"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"oauth2/trait.ErrorResponseType.html\" title=\"trait oauth2::ErrorResponseType\">ErrorResponseType</a>&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"oauth2/struct.ErrorResponse.html\" title=\"struct oauth2::ErrorResponse\">ErrorResponse</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"oauth2/trait.ErrorResponseType.html\" title=\"trait oauth2::ErrorResponseType\">ErrorResponseType</a>,&nbsp;</span>","synthetic":false,"types":["oauth2::ErrorResponse"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"oauth2/basic/enum.BasicTokenType.html\" title=\"enum oauth2::basic::BasicTokenType\">BasicTokenType</a>","synthetic":false,"types":["oauth2::basic::BasicTokenType"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"oauth2/basic/enum.BasicErrorResponseType.html\" title=\"enum oauth2::basic::BasicErrorResponseType\">BasicErrorResponseType</a>","synthetic":false,"types":["oauth2::basic::BasicErrorResponseType"]}];
implementors["serde_json"] = [{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_json/map/struct.Map.html\" title=\"struct serde_json::map::Map\">Map</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>&gt;","synthetic":false,"types":["serde_json::map::Map"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>","synthetic":false,"types":["serde_json::value::Value"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_json/struct.Number.html\" title=\"struct serde_json::Number\">Number</a>","synthetic":false,"types":["serde_json::number::Number"]}];
implementors["serde_yaml"] = [{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_yaml/mapping/struct.Mapping.html\" title=\"struct serde_yaml::mapping::Mapping\">Mapping</a>","synthetic":false,"types":["serde_yaml::mapping::Mapping"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_yaml/struct.Number.html\" title=\"struct serde_yaml::Number\">Number</a>","synthetic":false,"types":["serde_yaml::number::Number"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"serde_yaml/enum.Value.html\" title=\"enum serde_yaml::Value\">Value</a>","synthetic":false,"types":["serde_yaml::value::Value"]}];
implementors["tmc_langs_abstraction"] = [{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tmc_langs_abstraction/enum.Strategy.html\" title=\"enum tmc_langs_abstraction::Strategy\">Strategy</a>","synthetic":false,"types":["tmc_langs_abstraction::Strategy"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tmc_langs_abstraction/struct.ValidationError.html\" title=\"struct tmc_langs_abstraction::ValidationError\">ValidationError</a>","synthetic":false,"types":["tmc_langs_abstraction::ValidationError"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tmc_langs_abstraction/struct.ValidationResult.html\" title=\"struct tmc_langs_abstraction::ValidationResult\">ValidationResult</a>","synthetic":false,"types":["tmc_langs_abstraction::ValidationResult"]}];
implementors["tmc_langs_core"] = [{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tmc_langs_core/struct.Organization.html\" title=\"struct tmc_langs_core::Organization\">Organization</a>","synthetic":false,"types":["tmc_langs_core::response::Organization"]}];
implementors["tmc_langs_framework"] = [{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tmc_langs_framework/domain/struct.TestDesc.html\" title=\"struct tmc_langs_framework::domain::TestDesc\">TestDesc</a>","synthetic":false,"types":["tmc_langs_framework::domain::TestDesc"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tmc_langs_framework/domain/struct.TestResult.html\" title=\"struct tmc_langs_framework::domain::TestResult\">TestResult</a>","synthetic":false,"types":["tmc_langs_framework::domain::TestResult"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tmc_langs_framework/domain/struct.ExerciseDesc.html\" title=\"struct tmc_langs_framework::domain::ExerciseDesc\">ExerciseDesc</a>","synthetic":false,"types":["tmc_langs_framework::domain::ExerciseDesc"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tmc_langs_framework/domain/struct.RunResult.html\" title=\"struct tmc_langs_framework::domain::RunResult\">RunResult</a>","synthetic":false,"types":["tmc_langs_framework::domain::RunResult"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tmc_langs_framework/domain/enum.RunStatus.html\" title=\"enum tmc_langs_framework::domain::RunStatus\">RunStatus</a>","synthetic":false,"types":["tmc_langs_framework::domain::RunStatus"]},{"text":"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tmc_langs_framework/domain/struct.ExercisePackagingConfiguration.html\" title=\"struct tmc_langs_framework::domain::ExercisePackagingConfiguration\">ExercisePackagingConfiguration</a>","synthetic":false,"types":["tmc_langs_framework::domain::ExercisePackagingConfiguration"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()