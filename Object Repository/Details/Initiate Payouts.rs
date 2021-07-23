<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Initiate Payouts</name>
   <tag></tag>
   <elementGuidId>8fd42822-2bbc-4e6c-98cf-a09745ffa21f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;source_account\&quot;: \&quot;9100917191\&quot;,\n    \&quot;PIN\&quot;: \&quot;0047\&quot;,\n    \&quot;beneficiary_account_name\&quot;: \&quot;Tosin Yadeka\&quot;,\n    \&quot;beneficiary_nuban\&quot;: \&quot;1227140382\&quot;,\n    \&quot;beneficiary_bank_code\&quot;: \&quot;000014\&quot;,\n    \&quot;bank_code_scheme\&quot;: \&quot;NIP\&quot;,\n    \&quot;currency_code\&quot;: \&quot;NGN\&quot;,\n    \&quot;narration\&quot;: \&quot;For kitchen utensils\&quot;,\n    \&quot;amount\&quot;: 10,\n    \&quot;reference\&quot;: \&quot;payout_48369228856415\&quot;,\n    \&quot;callback_url\&quot;: \&quot;\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_secret</name>
      <type>Main</type>
      <value>vb_ts_bce915031c0dc04a255ec06e380d9621a15dd47fd399</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.woven.finance/v2/api/payouts/request?command=initiate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
