<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddShippingRate</name>
   <tag></tag>
   <elementGuidId>a4895dc4-128f-4e80-aa81-18f180dd9b5d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${shipping-method-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addShippingRate\&quot;,\n            \&quot;zone\&quot; : {\n              \&quot;typeId\&quot; : \&quot;zone\&quot;,\n              \&quot;id\&quot; : \&quot;${zone-id}\&quot;\n            },\n            \&quot;shippingRate\&quot; : {\n              \&quot;price\&quot; : {\n                \&quot;currencyCode\&quot; : \&quot;EUR\&quot;,\n                \&quot;centAmount\&quot; : 4000\n              }\n            }\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>49b9e497-fa0f-497a-a604-0f8142f446d1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>cac1f4e5-e23f-4377-869a-220b2ae2a0fe</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/shipping-methods/${shipping-method-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>e02542b4-15ad-4fb3-8072-506ba557d40f</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>0e421951-1e7c-4a22-83e3-0715f23ff6e7</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.shipping-method-id</defaultValue>
      <description></description>
      <id>d8291f6e-d56b-430b-b3a2-d131cda22fac</id>
      <masked>false</masked>
      <name>shipping-method-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.shipping-method-version</defaultValue>
      <description></description>
      <id>fb910241-c9be-48ed-bb00-25389b42be1b</id>
      <masked>false</masked>
      <name>shipping-method-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.zone-id</defaultValue>
      <description></description>
      <id>ff1220d9-974f-4583-97a3-7b2e330b944b</id>
      <masked>false</masked>
      <name>zone-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
